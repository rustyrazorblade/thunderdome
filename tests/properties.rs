extern crate thunderdome;

#[cfg(test)]
mod property_tests {

    use thunderdome::graph::*;
    use thunderdome::property::*;

    #[test]
    fn test_get_set_compare_properties() {

    }

    #[test]
    fn test_properties() {
        let mut g = Graph::new();
        let mut v1 = g.add_vertex();
        let key = "jon".to_string();
        let value = Property::Int(1);
        v1.set_property(key.clone(), value);

        let result = v1.get_property(key).unwrap();

        match result {
            &Property::Int(1) =>
                // we're ok
                println!("OK"),
            _ =>
                panic!("wrong graph property")
        }
    }

    #[test]
    fn test_has() {
        let mut g = Graph::new();
        let mut v = g.add_vertex();
        let mut v2 = g.add_vertex();
        v.add_edge(&mut v2, &"friends");

        v2.set_property("favorite_food".to_string(), Property::String("pizza".to_string()));

        let result = g.v(1).outV(&[&"friends"]).
                        has(&"favorite_food",
                            Property::String("pizza".to_string()));

        // assert_eq!(result.paths.len(), 1);

    }

}
