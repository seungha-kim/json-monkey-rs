use crate::ast::AstNode;
use crate::environment::Environment;
use crate::jir::{JirParser, ParseError};
use crate::value::Value;

pub struct Interpreter {
    global_env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            global_env: Environment::new(),
        }
    }

    pub fn eval_str(&mut self, s: &str) -> Result<Value, EvalError> {
        // TODO: err
        let node = JirParser::parse_json(s)?;
        self.eval(&node)
    }

    pub fn eval(&mut self, ast: &AstNode) -> Result<Value, EvalError> {
        match ast {
            AstNode::Literal(value) => Ok(value.clone()),
            AstNode::Add(lhs, rhs) => self.zip_pair_with(lhs, rhs, |l, r| {
                Value::Number(l.as_number().unwrap() + r.as_number().unwrap())
            }),
            AstNode::Sub(lhs, rhs) => self.zip_pair_with(lhs, rhs, |l, r| {
                Value::Number(l.as_number().unwrap() - r.as_number().unwrap())
            }),
            AstNode::Ident(ident) => {
                // TODO: without clone
                self.global_env
                    .bindings
                    .get(ident)
                    .cloned()
                    .ok_or(EvalError::UndefinedIdent)
            }
            AstNode::Bind(ident, ast) => {
                let value = self.eval(ast)?;
                self.global_env.bindings.insert(ident.clone(), value);
                Ok(Value::Null)
            }
        }
    }

    fn zip_pair_with<F>(&mut self, lhs: &AstNode, rhs: &AstNode, f: F) -> Result<Value, EvalError>
    where
        F: FnOnce(&Value, &Value) -> Value,
    {
        vec![self.eval(&lhs), self.eval(&rhs)]
            .into_iter()
            .collect::<Result<Vec<Value>, EvalError>>()
            .map(|values| f(&values[0], &values[1]))
    }
}

#[derive(Debug)]
pub enum EvalError {
    ParseError(ParseError),
    UndefinedIdent,
}

impl From<ParseError> for EvalError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Ident;

    #[test]
    fn it_evaluate_addition() {
        let mut i = Interpreter::new();
        let ast = AstNode::Add(
            Box::new(AstNode::Literal(Value::Number(1.0))),
            Box::new(AstNode::Literal(Value::Number(2.0))),
        );
        let result = i.eval(&ast);
        assert_eq!(result.unwrap().as_number().unwrap(), &3.0)
    }

    #[test]
    fn it_evaluate_subtraction() {
        let mut i = Interpreter::new();
        let ast = AstNode::Sub(
            Box::new(AstNode::Literal(Value::Number(2.0))),
            Box::new(AstNode::Literal(Value::Number(1.0))),
        );
        let result = i.eval(&ast);
        assert_eq!(result.unwrap().as_number().unwrap(), &1.0)
    }

    #[test]
    fn it_evaluate_binding_and_ident() {
        let mut i = Interpreter::new();

        i.eval(&AstNode::Bind(
            Ident("foo".into()),
            Box::new(AstNode::Literal(Value::Number(1.0))),
        ))
        .unwrap();

        assert_eq!(
            i.eval(&AstNode::Ident(Ident("foo".into())))
                .unwrap()
                .as_number()
                .unwrap(),
            &1.0
        );

        assert_eq!(
            i.eval(&AstNode::Add(
                Box::new(AstNode::Ident(Ident("foo".into()))),
                Box::new(AstNode::Ident(Ident("foo".into())))
            ))
            .unwrap()
            .as_number()
            .unwrap(),
            &2.0
        );
    }
}
