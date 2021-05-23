use crate::value::Value;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub enum AstNode {
    Literal(Value),
    Ident(Ident),

    Add(Box<AstNode>, Box<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),

    And(Box<AstNode>, Box<AstNode>),
    Or(Box<AstNode>, Box<AstNode>),
    Not(Box<AstNode>),

    Eq(Box<AstNode>, Box<AstNode>),
    NotEq(Box<AstNode>, Box<AstNode>),

    If(Box<AstNode>, Box<AstNode>, Option<Box<AstNode>>),

    Bind(Ident, Box<AstNode>),
}
