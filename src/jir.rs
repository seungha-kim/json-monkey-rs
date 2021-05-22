use crate::ast::AstNode;
use crate::error::EvalError;
use serde_json::Value as JsonValue;
use crate::value::Value;

pub struct JirParser {}

pub type ParseResult = Result<AstNode, EvalError>;

impl JirParser {
    pub fn parse_json(json_str: &str) -> ParseResult {
        let value: serde_json::Value = serde_json::from_str(json_str)?;

        Self::parse_expression(&value)
    }

    fn parse_expression(json: &JsonValue) -> ParseResult {
        match json {
            JsonValue::Array(values) => {
                Self::parse_compound(values)
            }
            JsonValue::Number(num) => {
                Ok(AstNode::Literal(Value::Number(num.as_f64().unwrap())))
            }
            _ => unimplemented!()
        }
    }

    fn parse_compound(vs: &[JsonValue]) -> ParseResult {
        match &vs[0] {
            JsonValue::String(s) if s == "$add" => {
                vs[1..=2]
                    .iter()
                    .map(Self::parse_expression)
                    .collect::<Result<Vec<AstNode>, EvalError>>()
                    .map(|ns| {
                        AstNode::Add(Box::new(ns[0].clone()), Box::new(ns[1].clone()))
                    })
            }
            JsonValue::String(s) if s == "$sub" => {
                vs[1..=2]
                    .iter()
                    .map(Self::parse_expression)
                    .collect::<Result<Vec<AstNode>, EvalError>>()
                    .map(|ns| {
                        AstNode::Sub(Box::new(ns[0].clone()), Box::new(ns[1].clone()))
                    })
            }
            _ => unimplemented!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_addition() {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$add", 1, 2]"#).unwrap());
        let expected = format!("{:?}", AstNode::Add(Box::new(AstNode::Literal(Value::Number(1.0))), Box::new(AstNode::Literal(Value::Number(2.0)))));
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_parses_subtraction() {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$sub", 1, 2]"#).unwrap());
        let expected = format!("{:?}", AstNode::Sub(Box::new(AstNode::Literal(Value::Number(1.0))), Box::new(AstNode::Literal(Value::Number(2.0)))));
        assert_eq!(actual, expected);
    }
}