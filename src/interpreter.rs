use crate::ast::{AstNode, Ident};
use crate::environment::Environment;
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

    pub fn eval(&mut self, ast: &AstNode) -> Result<Value, EvalError> {
        match ast {
            AstNode::Literal(value) => Ok(value.clone()),
            AstNode::Add(lhs, rhs) => {
                let lv = self.eval(lhs)?;
                let rv = self.eval(rhs)?;
                match (lv, rv) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                    _ => Err(EvalError::UnexpectedTypeForOperation),
                }
            }
            AstNode::Sub(lhs, rhs) => {
                let lv = self.eval(lhs)?;
                let rv = self.eval(rhs)?;
                Ok(Value::Number(lv.to_number()? - rv.to_number()?))
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

impl Value {
    pub fn to_number(&self) -> Result<f64, EvalError> {
        if let Value::Number(num) = self {
            Ok(num.clone())
        } else {
            Err(EvalError::UnsupportedConversion)
        }
    }

    pub fn to_string(&self) -> Result<String, EvalError> {
        match self {
            Value::Number(n) => Ok(n.to_string()),
            Value::String(s) => Ok(s.clone()),
            Value::Null => Ok("null".into()),
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    UnsupportedConversion,
    UndefinedIdent(Ident),
    UnexpectedTypeForOperation,
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
        assert_eq!(result?.to_number()?, 3.0);
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
        assert_eq!(result?.to_number()?, 1.0);
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
            i.eval(&AstNode::Ident(Ident("foo".into())))?.to_number()?,
            1.0
        );

        assert_eq!(
            i.eval(&AstNode::Add(
                Box::new(AstNode::Ident(Ident("foo".into()))),
                Box::new(AstNode::Ident(Ident("foo".into())))
            ))?
            .to_number()?,
            2.0
        );
        Ok(())
    }

    #[test]
    fn it_concatenate_strings() -> Result<(), EvalError> {
        let mut i = Interpreter::new();
        assert_eq!(
            i.eval(&AstNode::Add(
                Box::new(AstNode::Literal(Value::String("foo".into()))),
                Box::new(AstNode::Literal(Value::String("bar".into()))),
            ))?
            .to_string()?,
            "foobar".to_string()
        );

        Ok(())
    }
}
