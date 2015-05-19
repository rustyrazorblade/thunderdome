extern crate thunderdome;

#[cfg(test)]
mod traversal_tests {

    use thunderdome::graph::*;

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




}
