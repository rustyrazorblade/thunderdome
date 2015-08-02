use std::any::Any;
use std::collections::HashMap;
use std::clone::Clone;

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

#[derive(Debug)]
pub struct PropertyMap {
    props: HashMap<String, Box<Any>>
}
impl PropertyMap {
    pub fn new() -> Self {
        PropertyMap{ props: HashMap::new() }
    }
}
// pub type PropertyMap = HashMap<String, Box<Any>>;

impl Clone for PropertyMap {
    fn clone(&self) -> Self {
        println!("TODO: actually clone the map")
        PropertyMap::new()
    }

    fn clone_from(&mut self, source: &Self) {

    }

}
