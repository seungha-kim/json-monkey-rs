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
            AstNode::If(cond, true_branch, false_branch) => {
                if self.eval(cond)?.to_boolean()? {
                    Ok(self.eval(true_branch)?)
                } else {
                    if let Some(fb) = false_branch {
                        self.eval(fb)
                    } else {
                        Ok(Value::Null)
                    }
                }
            }
            AstNode::And(lhs, rhs) => {
                let lv = self.eval(lhs)?.to_boolean()?;
                let rv = self.eval(rhs)?.to_boolean()?;
                Ok(Value::Boolean(lv && rv))
            }
            AstNode::Or(lhs, rhs) => {
                let lv = self.eval(lhs)?.to_boolean()?;
                let rv = self.eval(rhs)?.to_boolean()?;
                Ok(Value::Boolean(lv || rv))
            }
            AstNode::Not(arg) => Ok(Value::Boolean(!self.eval(arg)?.to_boolean()?)),
            AstNode::Eq(lhs, rhs) => {
                let lv = self.eval(lhs)?;
                let rv = self.eval(rhs)?;
                Ok(Value::Boolean(lv == rv))
            }
            AstNode::NotEq(lhs, rhs) => {
                let lv = self.eval(lhs)?;
                let rv = self.eval(rhs)?;
                Ok(Value::Boolean(lv != rv))
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
            Value::Boolean(b) => Ok(b.to_string()),
        }
    }

    pub fn to_boolean(&self) -> Result<bool, EvalError> {
        match self {
            Value::Boolean(b) => Ok(b.clone()),
            Value::Number(n) => Ok(*n != 0.0),
            Value::String(s) => Ok(!s.is_empty()),
            Value::Null => Ok(false),
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

    #[test]
    fn it_evaluates_if_expression() -> Result<(), EvalError> {
        let mut i = Interpreter::new();
        assert_eq!(
            i.eval(&AstNode::If(
                Box::new(AstNode::Literal(Value::Boolean(true))),
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Some(Box::new(AstNode::Literal(Value::Number(2.0)))),
            ))?,
            Value::Number(1.0)
        );

        Ok(())
    }

    #[test]
    fn it_evaluates_if_expression_as_null() -> Result<(), EvalError> {
        let mut i = Interpreter::new();
        assert_eq!(
            i.eval(&AstNode::If(
                Box::new(AstNode::Literal(Value::Boolean(false))),
                Box::new(AstNode::Literal(Value::Number(1.0))),
                None,
            ))?,
            Value::Null,
        );

        Ok(())
    }

    #[test]
    fn it_evaluates_non_boolean_types_as_boolean() -> Result<(), EvalError> {
        assert_eq!(Value::Number(0.0).to_boolean()?, false);
        assert_eq!(Value::Number(1.0).to_boolean()?, true);
        assert_eq!(Value::Number(-1.0).to_boolean()?, true);

        assert_eq!(Value::String("".into()).to_boolean()?, false);
        assert_eq!(Value::String("nonempty".into()).to_boolean()?, true);

        assert_eq!(Value::Null.to_boolean()?, false);

        Ok(())
    }

    #[test]
    fn it_evaluates_boolean_operation() -> Result<(), EvalError> {
        let mut i = Interpreter::new();

        assert_eq!(
            i.eval(&AstNode::And(
                Box::new(AstNode::Literal(Value::Boolean(true))),
                Box::new(AstNode::Literal(Value::Boolean(false)))
            ))?,
            Value::Boolean(false)
        );

        assert_eq!(
            i.eval(&AstNode::Or(
                Box::new(AstNode::Literal(Value::Boolean(true))),
                Box::new(AstNode::Literal(Value::Boolean(false)))
            ))?,
            Value::Boolean(true)
        );

        assert_eq!(
            i.eval(&AstNode::Not(Box::new(AstNode::Literal(Value::Boolean(
                true
            ))),))?,
            Value::Boolean(false)
        );

        Ok(())
    }

    #[test]
    fn it_evaluates_comparison_operation() -> Result<(), EvalError> {
        let mut i = Interpreter::new();

        assert_eq!(
            i.eval(&AstNode::Eq(
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Box::new(AstNode::Literal(Value::Number(1.0)))
            ))?,
            Value::Boolean(true)
        );

        assert_eq!(
            i.eval(&AstNode::NotEq(
                Box::new(AstNode::Literal(Value::Number(1.0))),
                Box::new(AstNode::Literal(Value::Number(1.0)))
            ))?,
            Value::Boolean(false)
        );

        Ok(())
    }
}
