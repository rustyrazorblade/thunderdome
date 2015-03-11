use std::mem;
use std::collections::HashMap;
use std::mem::transmute;

struct Graph {
	vertices: HashMap<i64, *mut Vertex>,
}

impl Graph {
	fn new() -> Box<Graph> {
		let hm: HashMap<i64, *mut Vertex> = HashMap::new();
		Box::new(Graph{vertices:hm})
	}
	fn add_vertex(&mut self) -> *mut Vertex {
		let mut v = Box::new(Vertex{id:0});

		unsafe {
			let foo: *mut Vertex = transmute(v);
			foo
		}

	}
}
struct Vertex {
	id: i32,


}

struct Edge {
	id: i32,
}

fn main() {
	let mut g = Graph::new();

    println!("Hello, world!");
}

#[test]
fn test_unsafe_vertex() {
	let mut g = Graph::new();
	g.add_vertex();
	g.add_vertex();

}
