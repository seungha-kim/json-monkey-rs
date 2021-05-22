use crate::interpreter::{EvalError, Interpreter};
use crate::jir::{JirParser, ParseError};
use crate::value::Value;

pub struct Repl {
    interpreter: Interpreter,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn eval_str(&mut self, s: &str) -> Result<Value, ReplError> {
        let node = JirParser::parse_json(s)?;
        Ok(self.interpreter.eval(&node)?)
    }
}

#[derive(Debug)]
pub enum ReplError {
    ParseError(ParseError),
    EvalError(EvalError),
}

impl From<ParseError> for ReplError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

impl From<EvalError> for ReplError {
    fn from(e: EvalError) -> Self {
        Self::EvalError(e)
    }
}
