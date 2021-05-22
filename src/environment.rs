use std::collections::HashMap;

use crate::ast::Ident;
use crate::value::Value;

pub struct Environment {
    pub bindings: HashMap<Ident, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}
