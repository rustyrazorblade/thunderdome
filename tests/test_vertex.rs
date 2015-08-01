extern crate thunderdome;

#[cfg(test)]
mod vertex_tests {
    use thunderdome::graph::*;
    use thunderdome::vertex::*;

    #[test]
    fn test_vertex_edge_traversals() {
        let mut g = Graph::new();
        let mut vertex = g.add_vertex();
        let mut vertex2 = g.add_vertex();

        g.add_edge(&mut vertex, &mut vertex2, "test");

        let result = vertex.read().unwrap().out_edges().len();
        assert_eq!(1, result);

        let result = vertex.read().unwrap().in_edges().len();
        assert_eq!(0, result);

        let result = vertex2.read().unwrap().out_edges().len();
        assert_eq!(0, result);

        let result = vertex2.read().unwrap().in_edges().len();
        assert_eq!(1, result);

        for edge in vertex.read().unwrap().out_edges().iter() {
            println!("hi {}", edge.read().unwrap().label);
        }
    }


    #[test]
    fn test_new_vertex() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();

        assert_eq!(v1.read().unwrap().id, 1);

        let v2 = g.add_vertex();
        assert!(v2.read().unwrap().id == 2);
    }

}
