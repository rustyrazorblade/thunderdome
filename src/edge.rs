use std::ops::Deref;

use vertex::{RawVertex, Vertex};
use graph::TraversableToVertex;

#[derive(Debug)]
pub struct RawEdge {
    pub from_vertex: *mut RawVertex,
    pub to_vertex: *mut RawVertex,
    pub label: String
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub edge: *mut RawEdge
}

impl Deref for Edge {
    type Target = RawEdge;
    fn deref<'a>(&'a self) -> &'a RawEdge {
        unsafe { &*(self.edge) }
    }
}

impl TraversableToVertex for Edge {
	fn inV(&self) -> Vec<Vertex> {
		let mut result = Vec::new();
		unsafe {
			let vertex: &RawVertex = &*(self.to_vertex);
			let proxy = Vertex{id:vertex.id, v:self.to_vertex};
			result.push(proxy);
		}
		result
	}
	fn outV(&self, labels: &[&str]) -> Vec<Vertex> {
		let mut result = Vec::new();
		unsafe {
			let vertex: &RawVertex = &*(self.from_vertex);
			let proxy = Vertex{id:vertex.id, v:self.from_vertex};
			result.push(proxy);
		}
		result
	}
}
