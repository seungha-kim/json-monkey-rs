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
            JsonValue::Number(num) => {
                if let Some(n) = num.as_f64() {
                    Ok(AstNode::Literal(Value::Number(n)))
                } else {
                    Err(ParseError::UnsupportedNumberLiteral(num.to_string()))
                }
            }
            JsonValue::Null => Ok(AstNode::Literal(Value::Null)),
            JsonValue::String(s) => Ok(AstNode::Literal(Value::String(s.clone()))),
            JsonValue::Bool(b) => Ok(AstNode::Literal(Value::Boolean(b.clone()))),
            _ => unimplemented!(),
        }
    }

    fn parse_compound(vs: &[JsonValue]) -> Result<AstNode, ParseError> {
        match &vs[0] {
            JsonValue::String(s) if s == "$add" => {
                Self::assert_form_range(vs, Some(3), Some(3))?;
                let lhs = Self::parse_expression(&vs[1])?;
                let rhs = Self::parse_expression(&vs[2])?;
                Ok(AstNode::Add(Box::new(lhs), Box::new(rhs)))
            }
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
            JsonValue::String(s) if s == "$if" => {
                Self::assert_form_range(vs, Some(3), Some(4))?;
                let cond = Self::parse_expression(&vs[1])?;
                let true_branch = Self::parse_expression(&vs[2])?;
                let false_branch = if vs.len() == 4 {
                    Some(Box::new(Self::parse_expression(&vs[3])?))
                } else {
                    None
                };
                Ok(AstNode::If(
                    Box::new(cond),
                    Box::new(true_branch),
                    false_branch,
                ))
            }
            JsonValue::String(s) if s == "$while" => {
                Self::assert_form_range(vs, Some(3), Some(3))?;
                let cond = Self::parse_expression(&vs[1])?;
                let body = Self::parse_expression(&vs[2])?;
                Ok(AstNode::While(Box::new(cond), Box::new(body)))
            }
            JsonValue::String(s) if s == "$and" => {
                Self::assert_form_range(vs, Some(3), Some(3))?;
                Ok(AstNode::And(
                    Box::new(Self::parse_expression(&vs[1])?),
                    Box::new(Self::parse_expression(&vs[2])?),
                ))
            }
            JsonValue::String(s) if s == "$or" => {
                Self::assert_form_range(vs, Some(3), Some(3))?;
                Ok(AstNode::Or(
                    Box::new(Self::parse_expression(&vs[1])?),
                    Box::new(Self::parse_expression(&vs[2])?),
                ))
            }
            JsonValue::String(s) if s == "$not" => {
                Self::assert_form_range(vs, Some(2), Some(2))?;
                Ok(AstNode::Not(Box::new(Self::parse_expression(&vs[1])?)))
            }
            JsonValue::String(s) if s == "$eq" => {
                Self::assert_form_range(vs, Some(3), Some(3))?;
                Ok(AstNode::Eq(
                    Box::new(Self::parse_expression(&vs[1])?),
                    Box::new(Self::parse_expression(&vs[2])?),
                ))
            }
            JsonValue::String(s) if s == "$notEq" => {
                Self::assert_form_range(vs, Some(3), Some(3))?;
                Ok(AstNode::NotEq(
                    Box::new(Self::parse_expression(&vs[1])?),
                    Box::new(Self::parse_expression(&vs[2])?),
                ))
            }
            _ => Err(ParseError::UnsupportedForm),
        }
    }

    fn parse_ident(v: &JsonValue) -> Result<Ident, ParseError> {
        match v {
            JsonValue::String(s) => Ok(Ident(s.clone())),
            _ => Err(ParseError::IdentExpected),
        }
    }

    fn assert_form_range(
        vs: &[JsonValue],
        min: Option<usize>,
        max: Option<usize>,
    ) -> Result<(), ParseError> {
        let expected_min = min.unwrap_or(usize::MIN);
        let expected_max = max.unwrap_or(usize::MAX);
        let actual = vs.len();
        if actual < expected_min {
            Err(ParseError::NotEnoughArgs {
                actual,
                expected_min,
            })
        } else if actual > expected_max {
            Err(ParseError::TooManyArgs {
                actual,
                expected_max,
            })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidJson(serde_json::Error),
    IdentExpected,
    TooManyArgs { actual: usize, expected_max: usize },
    NotEnoughArgs { actual: usize, expected_min: usize },
    InvalidFormLength { actual: usize, expected: String },
    UnsupportedNumberLiteral(String),
    UnsupportedForm,
}

impl From<serde_json::Error> for ParseError {
    fn from(e: Error) -> Self {
        ParseError::InvalidJson(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_addition() -> Result<(), ParseError> {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$add", 1, 2]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::Add(
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Box::new(AstNode::Literal(Value::Number(2.0)))
            )
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn it_parses_subtraction() -> Result<(), ParseError> {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$sub", 1, 2]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::Sub(
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Box::new(AstNode::Literal(Value::Number(2.0)))
            )
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn it_parses_binding() -> Result<(), ParseError> {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$bind", "foo", 1]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::Bind(
                Ident("foo".into()),
                Box::new(AstNode::Literal(Value::Number(1.0)))
            )
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn it_parses_ref() -> Result<(), ParseError> {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$ref", "foo"]"#)?);
        let expected = format!("{:?}", AstNode::Ident(Ident("foo".into()),));
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn it_parses_if_expression() -> Result<(), ParseError> {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$if", true, 1, 2]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::If(
                Box::new(AstNode::Literal(Value::Boolean(true))),
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Some(Box::new(AstNode::Literal(Value::Number(2.0))))
            )
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn it_parses_while_expression() -> Result<(), ParseError> {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$while", true, 1]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::While(
                Box::new(AstNode::Literal(Value::Boolean(true))),
                Box::new(AstNode::Literal(Value::Number(1.0))),
            )
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn it_parses_if_expression_without_false_branch() -> Result<(), ParseError> {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$if", true, 1]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::If(
                Box::new(AstNode::Literal(Value::Boolean(true))),
                Box::new(AstNode::Literal(Value::Number(1.0))),
                None,
            )
        );
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn it_parses_boolean_operation() -> Result<(), ParseError> {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$and", true, false]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::And(
                Box::new(AstNode::Literal(Value::Boolean(true))),
                Box::new(AstNode::Literal(Value::Boolean(false))),
            )
        );
        assert_eq!(actual, expected);

        let actual = format!("{:?}", JirParser::parse_json(r#"["$or", true, false]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::Or(
                Box::new(AstNode::Literal(Value::Boolean(true))),
                Box::new(AstNode::Literal(Value::Boolean(false))),
            )
        );
        assert_eq!(actual, expected);

        let actual = format!("{:?}", JirParser::parse_json(r#"["$not", true]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::Not(Box::new(AstNode::Literal(Value::Boolean(true))))
        );
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn it_parses_comparison_operation() -> Result<(), ParseError> {
        let actual = format!("{:?}", JirParser::parse_json(r#"["$eq", 1.0, 2.0]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::Eq(
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Box::new(AstNode::Literal(Value::Number(2.0))),
            )
        );
        assert_eq!(actual, expected);

        let actual = format!("{:?}", JirParser::parse_json(r#"["$notEq", 1.0, 2.0]"#)?);
        let expected = format!(
            "{:?}",
            AstNode::NotEq(
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Box::new(AstNode::Literal(Value::Number(2.0))),
            )
        );
        assert_eq!(actual, expected);

        Ok(())
    }
}
