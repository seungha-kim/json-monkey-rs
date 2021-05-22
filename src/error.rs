use serde_json::Error;

#[derive(Debug)]
pub enum EvalError {
    InvalidJson(serde_json::Error),
}

impl From<serde_json::Error> for EvalError {
    fn from(e: Error) -> Self {
        EvalError::InvalidJson(e)
    }
}

