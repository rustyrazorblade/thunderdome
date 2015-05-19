#[derive(Debug)]
pub enum Property {
    Int(i64),
    Float(f64),
    String(String),
}

pub enum Comparison {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual
}
