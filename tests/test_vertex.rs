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

        vertex.add_edge(&mut vertex2, "test");

        let result = vertex.outE();
        assert_eq!(1, result.len());

        let result = vertex.inE();
        assert_eq!(0, result.len());

        let result = vertex2.outE();
        assert_eq!(0, result.len());

        let result = vertex2.inE();
        assert_eq!(1, result.len());
    }


    #[test]
    fn test_new_vertex() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();

        assert_eq!(v1.id, 1);
        assert_eq!(v1.read().unwrap().id, 1);

        let v2 = g.add_vertex();
        assert!(v2.read().unwrap().id == 2);
    }

}
