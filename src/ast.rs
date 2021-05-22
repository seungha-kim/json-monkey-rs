use crate::value::Value;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub enum AstNode {
    Literal(Value),
    Ident(Ident),

    Add(Box<AstNode>, Box<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),

    Bind(Ident, Box<AstNode>),
}
