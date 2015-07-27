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


		g.add_edge(&mut jon, &mut blake, "friend");
		g.add_edge(&mut jon, &mut caleb, "friend");
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
    fn test_real_out_vertex_execution() {
        let mut g = simple_graph();

        let result: GraphQueryResult = g.execute("g.v(1)").unwrap();
        // tree should have a root, then a single child
        let x = result.tree.children;
        assert!(x.len() == 1);

        let result = g.execute("g.v(1).outV('friend')").unwrap();

        let x = result.tree.children;
        // should have a single child - the 1
        assert_eq!(x.len(), 1);

        let ref tree = x.get(&0).unwrap();

        match tree.element {
            Element::Vertex(ref vertex) =>
                assert_eq!(vertex.id, 1),
            _ =>
                panic!("WRONG")
        };
        // let friends = tree.children.clone().unwrap();
        // let friends = x.children.unwrap();

    }



}
