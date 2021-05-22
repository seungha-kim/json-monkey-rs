#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Number(f64),
    String(String),
}
