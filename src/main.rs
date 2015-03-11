use std::mem;
use std::collections::HashMap;

struct Graph {
	vertices: HashMap<i64, Vertex>,
}

impl Graph {
	fn new() -> Box<Graph> {
		let hm: HashMap<i64, Vertex> = HashMap::new();
		Box::new(Graph{vertices:hm})
	}
	fn add_vertex(&mut self) -> Vertex {
		let mut v = Vertex{id:0};
		v
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
