#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Number(f64),
    String(String),
}

impl Value {
    pub fn as_number(&self) -> Option<&f64> {
        if let Value::Number(num) = self {
            Some(num)
        } else {
            None
        }
    }
}
