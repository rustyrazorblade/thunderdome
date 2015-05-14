use std::ops::Deref;

use vertex::{RawVertex, Vertex};
use graph::TraversableToVertex;

#[derive(Debug)]
pub struct Edge {
    pub from_vertex: *mut RawVertex,
    pub to_vertex: *mut RawVertex
}

#[derive(Clone)]
pub struct EdgeProxy {
    pub edge: *mut Edge
}

impl Deref for EdgeProxy {
    type Target = Edge;
    fn deref<'a>(&'a self) -> &'a Edge {
        unsafe { &*(self.edge) }
    }
}

impl TraversableToVertex for EdgeProxy {
	fn inV(&self) -> &[Vertex] {
		let mut result = Vec::new();
		unsafe {
			let vertex: &RawVertex = &*(self.to_vertex);
			let proxy = Vertex{id:vertex.id, v:self.to_vertex};
			result.push(proxy);
		}
		result.as_slice()
	}
	fn outV(&self) -> &[Vertex] {
		let mut result = Vec::new();
		unsafe {
			let vertex: &RawVertex = &*(self.from_vertex);
			let proxy = Vertex{id:vertex.id, v:self.from_vertex};
			result.push(proxy);
		}
		result.as_slice()
	}
}
