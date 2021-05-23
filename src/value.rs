#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Number(f64),
    String(String),
    Boolean(bool),
}
