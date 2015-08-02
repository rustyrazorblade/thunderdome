use std::any::Any;
use std::collections::HashMap;
use std::clone::Clone;


trait HasPropertyMap {
    fn get_property_map() -> HashMap<String, Box<Any>>;
}
