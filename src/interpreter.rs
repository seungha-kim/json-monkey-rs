use crate::ast::AstNode;
use crate::value::Value;
use crate::jir::JirParser;

pub struct Interpreter {}

impl Interpreter {
    pub fn eval_str(s: &str) -> Result<Value, ()> {
        // TODO: err
        JirParser::parse_json(s).map_err(|_| ()).and_then(|node| Self::eval(&node))
    }

    pub fn eval(ast: &AstNode) -> Result<Value, ()> {
        match ast {
            AstNode::Literal(value) => Ok(value.clone()),
            AstNode::Add(lhs, rhs) => {
                Self::zip_pair_with(lhs, rhs, |l, r| Value::Number(l.as_number().unwrap() + r.as_number().unwrap()))
            }
            AstNode::Sub(lhs, rhs) => {
                Self::zip_pair_with(lhs, rhs, |l, r| Value::Number(l.as_number().unwrap() - r.as_number().unwrap()))
            }
        }
    }

    fn zip_pair_with<F>(lhs: &AstNode, rhs: &AstNode, f: F) -> Result<Value, ()> where F: FnOnce(&Value, &Value) -> Value {
        vec![Self::eval(&lhs), Self::eval(&rhs)]
            .into_iter()
            .collect::<Result<Vec<Value>, ()>>()
            .map(|values| {
                f(&values[0], &values[1])
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_evaluate_addition() {
        let ast = AstNode::Add(
            Box::new(AstNode::Literal(Value::Number(1.0))),
            Box::new(AstNode::Literal(Value::Number(2.0)))
        );
        let result = Interpreter::eval(&ast);
        assert_eq!(result.unwrap().as_number().unwrap(), &3.0)
    }

    #[test]
    fn it_evaluate_subtraction() {
        let ast = AstNode::Sub(
            Box::new(AstNode::Literal(Value::Number(2.0))),
            Box::new(AstNode::Literal(Value::Number(1.0)))
        );
        let result = Interpreter::eval(&ast);
        assert_eq!(result.unwrap().as_number().unwrap(), &1.0)
    }
}