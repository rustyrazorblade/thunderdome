extern crate thunderdome;

#[cfg(test)]
mod edge_tests {

    use thunderdome::graph::*;
    
    #[test]
    fn test_add_edge() {
        let mut g = Graph::new();
        let mut v1 = g.add_vertex();
        let mut v2 = g.add_vertex();

        let e1 = v1.add_edge(&mut v2, "test");
        v1.add_edge(&mut v2, "test");
        v1.add_edge(&mut v2, "test");

        assert_eq!(v1.out_edges.len(), v2.in_edges.len());

        // ensure the vertex and edge pointers are correct

        assert_eq!(v1.v, e1.from_vertex);
        assert_eq!(e1.to_vertex, v2.v);

        //assert_eq!(v1.v, *(e1.edge).from_vertex);
    }


}