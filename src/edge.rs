use std::ops::Deref;

use vertex::{Vertex, VertexProxy};
use graph::TraversableToVertex;

#[derive(Debug)]
pub struct Edge {
    pub from_vertex: *mut Vertex,
    pub to_vertex: *mut Vertex
}

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
	fn inV(&self) -> Vec<VertexProxy> {
		let mut result = Vec::new();
		unsafe {
			let vertex: &Vertex = &*(self.to_vertex);
			let proxy = VertexProxy{id:vertex.id, v:self.to_vertex};
			result.push(proxy);
		}
		result
	}
	fn outV(&self) -> Vec<VertexProxy> {
		let mut result = Vec::new();
		unsafe {
			let vertex: &Vertex = &*(self.from_vertex);
			let proxy = VertexProxy{id:vertex.id, v:self.from_vertex};
			result.push(proxy);
		}
		result
	}
}

