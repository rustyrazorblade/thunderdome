use std::ops::Deref;

use vertex::{Vertex, VertexPointer};
use graph::TraversableToVertex;
use std::rc::Rc;

#[derive(Debug)]
pub struct RawEdge {
    pub from_vertex: VertexPointer,
    pub to_vertex: VertexPointer,
    pub label: String
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub edge: Rc<Box<RawEdge>>
}

impl Edge {
    // creates a new edge
    // sets the in & out of it but doesn't touch the vertex
    pub fn new(from_vertex:VertexPointer, to_vertex: VertexPointer, label:String) -> Edge {
        let edge = Rc::new(Box::new(RawEdge{from_vertex:from_vertex,
                                      to_vertex:to_vertex,
                                      label:label}));
        Edge{edge:edge}
    }
}

impl Deref for Edge {
    type Target = RawEdge;
    fn deref<'a>(&'a self) -> &'a RawEdge {
        &self.edge
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
