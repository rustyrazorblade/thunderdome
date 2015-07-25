use std::ops::Deref;

use vertex::Vertex;
use graph::TraversableToVertex;
use std::rc::Rc;
use std::sync::RwLock;

pub type EdgePointer = Rc<Box<RwLock<Edge>>>;

#[derive(Clone, Debug)]
pub struct Edge {
    // pub edge: EdgePointer,
    pub from_vertex: Vertex,
    pub to_vertex: Vertex,
    pub label: String
}

impl Edge {
    // creates a new edge
    // sets the in & out of it but doesn't touch the vertex
    pub fn new(from_vertex:Vertex, to_vertex: Vertex, label:String) -> EdgePointer {
        let edge = Rc::new(Box::new(RwLock::new(Edge{from_vertex:from_vertex,
                                      to_vertex:to_vertex,
                                      label:label})));
        edge
    }
}

impl TraversableToVertex for Edge {
	fn inV(&self) -> Vec<Vertex> {
		let mut result = Vec::new();
		let proxy = self.to_vertex.clone();
		result.push(proxy);
		result
	}
	fn outV(&self, labels: &[&str]) -> Vec<Vertex> {
		let mut result = Vec::new();
		let vertex = self.from_vertex.clone();
        result.push(vertex);
		result
	}
}
