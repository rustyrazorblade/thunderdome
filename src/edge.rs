use std::ops::Deref;

use vertex::{Vertex, VertexPointer};
use graph::TraversableToVertex;
use std::rc::Rc;
use std::sync::RwLock;

pub type EdgePointer = Rc<RwLock<Edge>>;

#[derive(Clone, Debug)]
pub struct Edge {
    // pub edge: EdgePointer,
    pub from_vertex: VertexPointer,
    pub to_vertex: VertexPointer,
    pub label: String
}

impl Edge {
    // creates a new edge
    // sets the in & out of it but doesn't touch the vertex
    pub fn new(from_vertex:VertexPointer,
               to_vertex: VertexPointer,
               label:String) -> EdgePointer {
        let edge = Rc::new(RwLock::new(Edge{from_vertex:from_vertex,
                                            to_vertex:to_vertex,
                                            label:label}));
        edge
    }
}

impl TraversableToVertex for Edge {
	fn inV(&self) -> Vec<VertexPointer> {
		let mut result = Vec::new();
		let proxy = self.to_vertex.clone();
		result.push(proxy);
		result
	}
	fn outV(&self, labels: &[&str]) -> Vec<VertexPointer> {
		let mut result = Vec::new();
		let vertex = self.from_vertex.clone();
        result.push(vertex);
		result
	}
}
