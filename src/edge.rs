use std::ops::Deref;

use vertex::{RawVertex, Vertex};
use graph::TraversableToVertex;

use std::sync::Arc;
use std::cell::RefCell;


#[derive(Debug)]
pub struct RawEdge {
    pub from_vertex: *mut RawVertex,
    pub to_vertex: *mut RawVertex,
    pub label: String
}

impl RawEdge {
    fn new(from_vertex: *mut RawVertex,
           to_vertex: *mut RawVertex,
           label: &str) -> Arc<RefCell<Box<RawEdge>>> {

        Arc::new(
            RefCell::new(
            Box::new(
                RawEdge{from_vertex: from_vertex,
                to_vertex: to_vertex,
                label: label.to_string()})))
    }
}

#[derive(Clone)]
pub struct Edge {
    pub edge:  Arc<RefCell<Box<RawEdge>>>
}

impl Edge {
    fn new(from_vertex: &mut Vertex,
           to_vertex: &mut Vertex,
           label: &str) {

        let edge = RawEdge::new(from_vertex.v,
                                to_vertex.v,
                                label);
        // RawEdge{from_vertex:from_vertex.v,
        //                    to_vertex:to_vertex.v,
        //                    label: label};

        from_vertex.out_edges.push(edge);
        to_vertex.in_edges.push(edge);

    }
}

impl Deref for Edge {
    type Target = Arc<RefCell<Box<RawEdge>>>;

    fn deref<'a>(&'a self) -> &'a Arc<RefCell<Box<RawEdge>>> {
        self.edge
    }
}

impl TraversableToVertex for Edge {
	fn inV(&self) -> Vec<Vertex> {
		let mut result = Vec::new();
		let vertex: RawVertex = self.borrow_mut().to_vertex;
		let proxy = Vertex{id:vertex.id, v:self.borrow_mut().to_vertex};
		result.push(proxy);
		result
	}
	fn outV(&self, labels: &[&str]) -> Vec<Vertex> {
		let mut result = Vec::new();
		unsafe {
			let vertex: RawVertex = self.from_vertex.borrow_mut();
			let proxy = Vertex{id:vertex.id, v:self.from_vertex};
			result.push(proxy);
		}
		result
	}
}
