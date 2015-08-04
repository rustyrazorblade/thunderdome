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
        let value = Box::new(1);

        let mut vertex = v1.write().unwrap();
        vertex.set_property(&key, value);
        let prop = vertex.get_property(&key).unwrap();

        // assert_eq!(*prop, value);

        // match result {
        //     Property::Int(1) =>
        //         // we're ok
        //         println!("OK"),
        //     _ =>
        //         panic!("wrong graph property")
        // }
    }

    #[test]
    fn test_has() {
        let mut g = Graph::new();
        let mut v = g.add_vertex();
        let mut v2 = g.add_vertex();
        g.add_edge(&mut v, &mut v2, &"friends");

        // let pizza = Property::String("pizza".to_string());
        // v2.write().unwrap().set_property("favorite_food", pizza.clone());


    }

}
