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

    let (k, v) = ("key".to_string(), "Value".to_string());
    p.set(k, Box::new(v));

    {
        let tmp = p.get::<String>("key".to_string()).unwrap();
        assert_eq!(tmp, "Value");
    }

    p.set("key2".to_string(), Box::new(1));

    {

    }


}
