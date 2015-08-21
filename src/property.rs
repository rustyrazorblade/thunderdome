use std::collections::HashMap;
use std::any::Any;

#[derive(Debug)]
pub struct PropertyMap {
    props: HashMap<String, Box<Any>>
}

impl PropertyMap {
    pub fn set(&mut self, key: String, value: Box<Any>) {
        self.props.insert(key, value);
    }

    pub fn get<T:Any>(&self, key: String) -> Result<&T, &str> {
        let result = try!(self.props.get(&key).ok_or("not found"));
        let downcasted = result.downcast_ref::<T>().ok_or("type fail");
        downcasted
    }

    pub fn get_mut<T:Any>(&mut self, key: String) -> Result<&mut T, &str> {
        let mut result = try!(self.props.get_mut(&key).ok_or("not found"));
        let downcasted = result.downcast_mut::<T>().ok_or("type fail");
        downcasted
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
