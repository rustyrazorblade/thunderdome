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

    let v = "value".to_string();
    p.set(0, Property::String(v));
    p.set_string(1, "value".to_string());
    p.set_int(2, 1);

    // let v2 = p.get(k).unwrap();
    // assert_eq!(v2, k)
}

#[test]
fn test_map() {

}
