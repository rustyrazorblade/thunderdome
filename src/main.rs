use std::mem;
use std::collections::HashMap;
use std::mem::transmute;

struct Graph {
	vertices: HashMap<i64, Vertex>,
}

impl Graph {
	fn new() -> Box<Graph> {
		let hm: HashMap<i64, Vertex> = HashMap::new();
		Box::new(Graph{vertices:hm})
	}
	fn add_vertex(&mut self) -> VertexProxy {
		let new_id = 1;
		let vertex = Vertex{id:new_id};

		let ptr: *const Vertex = &vertex as *const Vertex;
		let mut v = Box::new(vertex);

		// return the proxy which knows it's own id
		VertexProxy{id:new_id, v: ptr}
	}
}

struct Vertex {
	id: i64,
}

impl Vertex {
	fn query(self) {

	}
}

struct VertexProxy {
	id: i64,
	v: *const Vertex,
}

struct Edge {
	id: i32,
	from_vertex: *const Vertex,
	to_vertex: *const Vertex
}

fn main() {
	let mut g = Graph::new();

    println!("Hello, world!");
}

#[test]
fn test_unsafe_vertex() {
	let mut g = Graph::new();
	let mut v1 = g.add_vertex();
	let mut v2 = g.add_vertex();

	v1.test()

}
