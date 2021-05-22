use serde_json::{Error, Value as JsonValue};

use crate::ast::{AstNode, Ident};
use crate::value::Value;

pub struct JirParser {}

impl JirParser {
    pub fn parse_json(json_str: &str) -> Result<AstNode, ParseError> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;

        Self::parse_expression(&value)
    }

    fn parse_expression(json: &JsonValue) -> Result<AstNode, ParseError> {
        match json {
            JsonValue::Array(values) => Self::parse_compound(values),
            JsonValue::Number(num) => Ok(AstNode::Literal(Value::Number(num.as_f64().unwrap()))),
            _ => unimplemented!(),
        }
    }

    fn parse_compound(vs: &[JsonValue]) -> Result<AstNode, ParseError> {
        match &vs[0] {
            JsonValue::String(s) if s == "$add" => vs[1..=2]
                .iter()
                .map(Self::parse_expression)
                .collect::<Result<Vec<AstNode>, ParseError>>()
                .map(|ns| AstNode::Add(Box::new(ns[0].clone()), Box::new(ns[1].clone()))),
            JsonValue::String(s) if s == "$sub" => vs[1..=2]
                .iter()
                .map(Self::parse_expression)
                .collect::<Result<Vec<AstNode>, ParseError>>()
                .map(|ns| AstNode::Sub(Box::new(ns[0].clone()), Box::new(ns[1].clone()))),
            JsonValue::String(s) if s == "$bind" => Ok(AstNode::Bind(
                Self::parse_ident(&vs[1])?,
                Box::new(Self::parse_expression(&vs[2])?),
            )),
            JsonValue::String(s) if s == "$ref" => Ok(AstNode::Ident(Self::parse_ident(&vs[1])?)),
            _ => unimplemented!(),
        }
    }

    fn parse_ident(v: &JsonValue) -> Result<Ident, ParseError> {
        match v {
            JsonValue::String(s) => Ok(Ident(s.clone())),
            _ => Err(ParseError::IdentExpected),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_addition() {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$add", 1, 2]"#).unwrap());
        let expected = format!(
            "{:?}",
            AstNode::Add(
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Box::new(AstNode::Literal(Value::Number(2.0)))
            )
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_parses_subtraction() {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$sub", 1, 2]"#).unwrap());
        let expected = format!(
            "{:?}",
            AstNode::Sub(
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Box::new(AstNode::Literal(Value::Number(2.0)))
            )
        );
        assert_eq!(actual, expected);
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidJson(serde_json::Error),
    IdentExpected,
}

impl From<serde_json::Error> for ParseError {
    fn from(e: Error) -> Self {
        ParseError::InvalidJson(e)
    }
}
