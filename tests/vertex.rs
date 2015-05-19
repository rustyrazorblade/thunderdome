extern crate thunderdome;

#[cfg(test)]
mod vertex_tests {
    use thunderdome::graph::*;
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

}
