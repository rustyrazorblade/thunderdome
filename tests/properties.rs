extern crate thunderdome;
use thunderdome::graph::*;
use thunderdome::property::*;

#[test]
fn test_get_set_compare_properties() {

}

#[test]
fn test_properties() {
    // let mut g = Graph::new();
    // let mut v1 = g.add_vertex();
    let mut p = PropertyMap::new();
    p.set("key".to_string(), Box::new("Value".to_string()));
    p.set("key2".to_string(), Box::new(1));

    let tmp = p.get::<String>("key".to_string()).unwrap();
    assert_eq!(tmp, "Value");


}
