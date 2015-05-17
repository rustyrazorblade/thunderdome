#![allow(raw_pointer_derive)]
// #![feature(collections)]

pub mod graph;
pub mod vertex;
pub mod edge;
pub mod traversal;
pub mod path;

#[cfg(test)]
mod tests {
    use vertex::{RawVertex, GraphProperty};
    use graph::{Graph, TraversableToVertex};
    use path::{Path, Element};

    #[test]
    fn test_unsafe_vertex() {
        let v = RawVertex::new(1);

        let out_edges = &v.out_edges;
        let in_edges = &v.in_edges;
    }

    #[test]
    fn test_deref() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();
        let vertex = &*v1;
        assert!(vertex.id == 1);
    }

    #[test]
    fn test_new_vertex() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();

        assert_eq!(v1.id, 1);

        unsafe {
            let vertex = &*v1;
            assert_eq!((*v1).id, 1);
        }

        let v2 = g.add_vertex();
        assert!(v2.id == 2);
    }

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

    #[test]
    fn test_properties() {
        let mut g = Graph::new();
        let mut v1 = g.add_vertex();
        let key = "jon".to_string();
        let value = GraphProperty::Int(1);
        v1.set_property(key.clone(), value);

        let result = v1.get_property(key).unwrap();

        match result {
            &GraphProperty::Int(1) =>
                // we're ok
                println!("OK"),
            _ =>
                panic!("wrong graph property")
        }
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
    fn test_permute_path() {
        let mut g = Graph::new();
        let mut v = g.add_vertex();
        let mut v2 = g.add_vertex();
        let mut v3 = g.add_vertex();

        let vec = vec![Element::Vertex(v2),
                       Element::Vertex(v3)];

        let mut path = Path::new(v);

        // check last path
        let path2 = path.permute(&vec);



    }
}
