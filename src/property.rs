use std::any::Any;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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

pub type PropertyMap = HashMap<String, Box<Any>>;
