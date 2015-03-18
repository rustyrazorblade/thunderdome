use std::mem;
use std::collections::HashMap;
use std::mem::transmute;

struct Graph {
	vertices: HashMap<i64, Box<Vertex>>,
}

impl Graph {
	fn new() -> Box<Graph> {
		let vertices: HashMap<i64, Box<Vertex>> = HashMap::new();
		Box::new(Graph{vertices:vertices})
	}
	fn add_vertex(&mut self) -> VertexProxy {
		let new_id = 1;
		let v = Vertex::new(new_id);

		let ptr: *const Box<Vertex> = &v as *const Box<Vertex>;

		self.vertices.insert(new_id, v);

		// return the proxy which knows it's own id
		VertexProxy{id:new_id, v: ptr}
	}
}

struct Vertex {
	id: i64,
	edges: *mut Vec<Edge>,
}

impl Vertex {
	fn new(id: i64) -> Box<Vertex> {

		let mut edges : Vec<Edge> = Vec::new();
		let edges_ptr :	*mut Vec<Edge> = &mut edges;
		let vertex = Vertex{id:id, edges:edges_ptr};
		let mut v = Box::new(vertex);
		return v;
	}

	fn query(self) {

	}
}

struct VertexProxy {
	id: i64,
	v: *const Box<Vertex>,
}

//let i: u32 = 1;
//// explicit cast
//let p_imm: *const u32 = &i as *const u32;
//let mut m: u32 = 2;
//// implicit coercion
//let p_mut: *mut u32 = &mut m;
//
//unsafe {
//let ref_imm: &u32 = &*p_imm;
//let ref_mut: &mut u32 = &mut *p_mut;
//}

impl VertexProxy {
	fn add_edge(& self, to_vertex: &VertexProxy) {
		unsafe {
			let in_vertex = &* self .v;
			let out_vertex = &*to_vertex.v;
		}
		// create the edge
		let e = Edge{from_vertex: self.v,
					 to_vertex:to_vertex.v};


	}
}

struct Edge {
	from_vertex: *const Box<Vertex>,
	to_vertex: *const Box<Vertex>
}

fn main() {
	let mut g = Graph::new();

    println!("Hello, world!");
}

#[test]
fn test_unsafe_vertex() {
	let mut g = Graph::new();
	let mut v1 = g.add_vertex();
}

#[test]
fn test_add_edge() {
	let mut g = Graph::new();
	let mut v1 = g.add_vertex();
	let mut v2 = g.add_vertex();
	v2.add_edge(&v2);

}
