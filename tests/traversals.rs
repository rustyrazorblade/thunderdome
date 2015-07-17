extern crate thunderdome;

#[cfg(test)]
mod traversal_tests {

    use thunderdome::graph::*;
    use thunderdome::path::Element;

    fn simple_graph() -> Box<Graph> {
        let mut g = Graph::new();
        let mut jon = g.add_vertex();
		let mut blake = g.add_vertex();
		let mut caleb = g.add_vertex();
		let mut mike = g.add_vertex();

		jon.add_edge(&mut blake, "friend");
		jon.add_edge(&mut caleb, "friend");
		blake.add_edge(&mut caleb, "friend");
		blake.add_edge(&mut jon, "tried_to_murder");

		jon.add_edge(&mut mike, "enemy");
        g
    }

    #[test]
    fn test_get_vertex() {
        let mut g = Graph::new();
        let vertex = g.add_vertex();
        let v = g.get(1).unwrap();
        assert_eq!(v.id, 1);
    }


    #[test]
    fn test_graph_query_off_vertex() {
        let mut g = Graph::new();
        let vertex = g.add_vertex();
        let query = g.v(1);
    }

    #[test]
    fn test_traversal_map() {
        let mut g = Graph::new();
        let mut vertex = g.add_vertex();
		let mut vertex2 = g.add_vertex();
		vertex.add_edge(&mut vertex2, "test");

        let result = g.v(1).outV(&["test"]);

		assert_eq!(result.paths.len(), 1);

        let result2 = g.v(1).outV(&["likes"]);
		assert_eq!(result2.paths.len(), 0);
    }

	#[test]
	fn test_vertex_outv() {
		let mut g = Graph::new();
		let mut vertex = g.add_vertex();
		let mut vertex2 = g.add_vertex();

		vertex.add_edge(&mut vertex2, "test");

		let result = vertex.outV(&["test"]);
		assert_eq!(1, result.len());

		// add another edge, check length
		let mut vertex3 = g.add_vertex();
		vertex.add_edge(&mut vertex3, "test");

		let result = vertex.outV(&["test"]);
		assert_eq!(2, result.len());

		let result = g.v(1).outV(&["test"]);

	}

    #[test]
    fn test_real_out_vertex_execution() {
        let mut g = simple_graph();

        let result: GraphQueryResult = g.execute("g.v(1)").unwrap();
        // tree should have a root, then a single child
        match result.tree.children {
            // should have a single child
            Some(x) =>
                // box/vec
                // assert!(result.tree. == Element::TreeRoot);
                assert!(x.len() == 1),
            _ =>
                panic!("FUCK")
        };


        let result = g.execute("g.v(1).outV('friend')");
    }



}
