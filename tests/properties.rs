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

    let k = "key".to_string();
    let v = "value".to_string();
    p.set(k, Property::String(v));
    // p.set_string(k, "value".to_string());
    // let v2 = p.get(k).unwrap();
    // assert_eq!(v2, k)
}
