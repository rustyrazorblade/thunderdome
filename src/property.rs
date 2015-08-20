use std::collections::HashMap;
use std::any::Any;

#[derive(Debug)]
pub struct PropertyMap {
    props: HashMap<String, Box<Any>>
}

impl PropertyMap {
    pub fn set(&mut self, key: String, value: Box<Any>) {

    }
    
    pub fn new() -> PropertyMap {
        PropertyMap{ props: HashMap::new() }
    }
}



impl Clone for PropertyMap {
    fn clone(&self) -> PropertyMap {
        PropertyMap{ props: HashMap::new() }
    }
    fn clone_from(&mut self, source: &PropertyMap)  {

    }
}
