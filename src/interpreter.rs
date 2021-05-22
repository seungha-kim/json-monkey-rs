use crate::ast::{AstNode, Ident};
use crate::environment::Environment;
use crate::jir::{JirParser, ParseError};
use crate::value::{TypeError, Value};

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
        let node = JirParser::parse_json(s)?;
        self.eval(&node)
    }

    pub fn eval(&mut self, ast: &AstNode) -> Result<Value, EvalError> {
        match ast {
            AstNode::Literal(value) => Ok(value.clone()),
            AstNode::Add(lhs, rhs) => {
                let lv = self.eval(lhs)?;
                let rv = self.eval(rhs)?;
                Ok(Value::Number(lv.as_number()? + rv.as_number()?))
            }
            AstNode::Sub(lhs, rhs) => {
                let lv = self.eval(lhs)?;
                let rv = self.eval(rhs)?;
                Ok(Value::Number(lv.as_number()? - rv.as_number()?))
            }
            AstNode::Ident(ident) => {
                // TODO: without clone
                self.global_env
                    .bindings
                    .get(ident)
                    .cloned()
                    .ok_or(EvalError::UndefinedIdent(ident.clone()))
            }
            AstNode::Bind(ident, ast) => {
                let value = self.eval(ast)?;
                self.global_env.bindings.insert(ident.clone(), value);
                Ok(Value::Null)
            }
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    ParseError(ParseError),
    TypeError(TypeError),
    UndefinedIdent(Ident),
}

impl From<ParseError> for EvalError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

impl From<TypeError> for EvalError {
    fn from(e: TypeError) -> Self {
        Self::TypeError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Ident;

    #[test]
    fn it_evaluate_addition() -> Result<(), EvalError> {
        let mut i = Interpreter::new();
        let ast = AstNode::Add(
            Box::new(AstNode::Literal(Value::Number(1.0))),
            Box::new(AstNode::Literal(Value::Number(2.0))),
        );
        let result = i.eval(&ast);
        assert_eq!(result?.as_number()?, 3.0);
        Ok(())
    }

    #[test]
    fn it_evaluate_subtraction() -> Result<(), EvalError> {
        let mut i = Interpreter::new();
        let ast = AstNode::Sub(
            Box::new(AstNode::Literal(Value::Number(2.0))),
            Box::new(AstNode::Literal(Value::Number(1.0))),
        );
        let result = i.eval(&ast);
        assert_eq!(result?.as_number()?, 1.0);
        Ok(())
    }

    #[test]
    fn it_evaluate_binding_and_ident() -> Result<(), EvalError> {
        let mut i = Interpreter::new();

        i.eval(&AstNode::Bind(
            Ident("foo".into()),
            Box::new(AstNode::Literal(Value::Number(1.0))),
        ))?;

        assert_eq!(
            i.eval(&AstNode::Ident(Ident("foo".into())))?.as_number()?,
            1.0
        );

        assert_eq!(
            i.eval(&AstNode::Add(
                Box::new(AstNode::Ident(Ident("foo".into()))),
                Box::new(AstNode::Ident(Ident("foo".into())))
            ))?
            .as_number()?,
            2.0
        );
        Ok(())
    }
}
