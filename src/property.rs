use std::collections::{HashMap, HashSet};
use std::any::Any;

#[derive(Debug, Clone, PartialEq)]
pub enum Property {
    Int(i64),
    Float(f64),
    String(String),
    Map(HashMap<String, Property>),
}

pub enum PropertyError {
    NotFound,
    TypeError
}


#[derive(Debug, Clone)]
pub struct PropertyMap {
    props: HashMap<String, Property>,
}

impl PropertyMap {
    // pub fn set(&mut self, key: String, value: Box<Any>) {
    //     self.props.insert(key, value);
    // }
    pub fn set(&mut self, key: String, value: Property) -> Option<Property> {
        self.props.insert(key, value)
    }
    pub fn set_string(&mut self, key: String, value: String) -> Option<Property> {
        self.set(key, Property::String(value))
    }
    pub fn set_int(&mut self, key: String, value: i64) -> Option<Property> {
        self.set(key, Property::Int(value))
    }
    pub fn get(&self, key: String) -> Option<&Property> {
        self.props.get(&key)
    }
    // pub fn get_int(&self, key:String) -> Result<&Property> {
    //     let p = try!(self.get(&key, ))
    // }
    // pub fn get<T:Any>(&self, key: String) -> Result<&T, &str> {
    //     let result = try!(self.props.get(&key).ok_or("not found"));
    //     let downcasted = result.downcast_ref::<T>().ok_or("type fail");
    //     downcasted
    // }
    //
    // pub fn get_mut<T:Any>(&mut self, key: String) -> Result<&mut T, &str> {
    //     let mut result = try!(self.props.get_mut(&key).ok_or("not found"));
    //     let downcasted = result.downcast_mut::<T>().ok_or("type fail");
    //     downcasted
    // }


    pub fn new() -> PropertyMap {
        PropertyMap{ props: HashMap::new() }
    }
}



// impl Clone for PropertyMap {
//     fn clone(&self) -> PropertyMap {
//         PropertyMap{ props: HashMap::new() }
//     }
//     fn clone_from(&mut self, source: &PropertyMap)  {
//
//     }
// }
