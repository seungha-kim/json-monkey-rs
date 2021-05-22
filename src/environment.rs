use std::collections::HashMap;
use crate::ast::{Ident, AstNode};
use crate::value::Value;

pub struct Environment {
    bindings: HashMap<Ident, Value>
}
