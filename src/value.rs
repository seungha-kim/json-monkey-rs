#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Number(f64),
    String(String),
}

impl Value {
    pub fn as_number(&self) -> Result<f64, TypeError> {
        if let Value::Number(num) = self {
            Ok(num.clone())
        } else {
            Err(TypeError::UnsupportedConversion)
        }
    }
}

#[derive(Debug)]
pub enum TypeError {
    UnsupportedConversion,
}
