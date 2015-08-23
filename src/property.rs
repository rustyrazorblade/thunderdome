

use std::collections::{HashMap, HashSet};
use std::any::Any;

use num::rational::BigRational;

#[derive(Debug, Clone)]
pub struct Schema {
    fields: HashMap<String, Field>,
}


#[derive(Debug, Clone, Copy)]
enum Type {
    Int,
    Decimal,
    String,
    Map,
    Set
}

#[derive(Debug, Clone)]
pub struct Field {
    id: i8,
    field_type: Type
}

impl Schema {
    pub fn new() -> Schema {
        Schema{fields: HashMap::new()}
    }
}



#[derive(Debug, Clone, PartialEq)]
pub enum Property {
    Int(i64),
    Decimal(BigRational),
    String(String),
    Map(HashMap<String, Property>),
    Set(HashSet<String>)
}

pub enum PropertyError {
    NotFound,
    TypeError
}


#[derive(Debug, Clone)]
pub struct PropertyMap {
    props: HashMap<i8, Property>,
}


impl PropertyMap {
    // pub fn set(&mut self, key: String, value: Box<Any>) {
    //     self.props.insert(key, value);
    // }
    pub fn set(&mut self, key: i8, value: Property) -> Option<Property> {
        self.props.insert(key, value)
    }
    pub fn set_string(&mut self, key: i8, value: String) -> Option<Property> {
        self.set(key, Property::String(value))
    }
    pub fn set_int(&mut self, key: i8, value: i64) -> Option<Property> {
        self.set(key, Property::Int(value))
    }
    pub fn get(&self, key: i8) -> Option<&Property> {
        self.props.get(&key)
    }

    pub fn get_int(&self, key:i8) -> Result<i64, PropertyError> {
        let p = try!(self.get(key).ok_or(PropertyError::NotFound));
        match *p {
            Property::Int(v) => Ok(v),
            _ => Err(PropertyError::TypeError)
        }
    }

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
