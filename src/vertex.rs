use std::ops::{Deref, DerefMut};
use std::mem;
use std::collections::HashMap;

#[derive(Debug)]
pub enum GraphProperty {
    Int(i64),
    Float(f64),
    String(String),
}

use edge::{RawEdge,Edge};
use graph::TraversableToVertex;

/*
* storing in & out edges seperately should cut down on the number of "things" i have to traverse
* if a vertex has 10k edges (5k in and out 5k out) then doing something like*   g(v).outV()
* should be ok
*/
pub struct RawVertex {
    pub id: i64,
    pub properties: HashMap<String, GraphProperty>,
    // pointers on both sides, yay
    pub out_edges: Vec<*mut RawEdge>,
    pub in_edges:  Vec<*mut RawEdge>,
}

impl RawVertex {
    pub fn new(id: i64) -> Box<RawVertex> {
        let mut props  = HashMap::new();
        Box::new(RawVertex{id:id,
                        properties: props,
                        out_edges: Vec::new(),
                        in_edges: Vec::new()})
    }



}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub id: i64,
    pub v: *mut RawVertex,
}

impl Vertex {

	pub fn new(v: *mut RawVertex) -> Vertex {
        let raw : &RawVertex = unsafe{ mem::transmute(v)};
        Vertex{id:raw.id, v:v}
	}

    pub fn add_edge(&mut self, to_vertex: &mut Vertex, label: &str) -> Edge {
        let in_vertex: &mut RawVertex;
        let out_vertex: &mut RawVertex;

        unsafe {
            in_vertex =  &mut *(self.v);
            out_vertex = &mut *(to_vertex.v);
        }

        // create the edge
        let e = Box::new(RawEdge{from_vertex: self.v,
                                 to_vertex: to_vertex.v,
                                 label: label.to_string() });

        // keep it on the heap but manage it myself
        let edge: *mut RawEdge = unsafe { mem::transmute(e) };

        in_vertex.out_edges.push(edge);
        out_vertex.in_edges.push(edge);
        Edge{edge:edge}
    }

    pub fn set_property(&mut self, field: String, value: GraphProperty) {
        self.properties.insert(field, value);
    }

    pub fn get_property(&self, field:String) -> Option<&GraphProperty> {
        self.properties.get(&field)
    }

    pub fn query(self) {
        unimplemented!()
    }

	pub fn outE(&self) -> Vec<Edge> {
		let mut result = Vec::new();
		unsafe {
			for &x in self.out_edges.iter() {
				let proxy = Edge{edge:x};
				result.push(proxy);
			}
		}
		result
	}

	pub fn inE(&self) -> Vec<Edge> {
		let mut result = Vec::new();
		for &x in self.in_edges.iter() {
			let proxy = Edge{edge:x};
			result.push(proxy);
		}
		result
	}

}

impl TraversableToVertex for Vertex {
	/* basic traversal operations
	   cannot be chained
	   look at traversal.rs for breadth first chaining
	*/

	/* returns all the outV vertex proxies
	   mainly for internal use
	*/
	fn outV(&self, labels: &[&str]) -> Vec<Vertex> {
		let mut result = Vec::new();
		unsafe {
			for &x in self.out_edges.iter() {
				let edge: &RawEdge = &*x;
				let vertex: &RawVertex = &*(edge.to_vertex);

                let label : &str = &edge.label;

                if labels.is_empty() || labels.contains(&label) {
    				let proxy = Vertex{id:vertex.id, v:edge.to_vertex};
    				result.push(proxy);
                }
			}
			result
		}
	}
	fn inV(&self) -> Vec<Vertex> {
		let mut result = Vec::new();
		unsafe {
			for &x in self.in_edges.iter() {
				let edge: &RawEdge = &*x;
				let vertex: &RawVertex = &*(edge.from_vertex);

				let proxy = Vertex{id:vertex.id, v:edge.from_vertex};
				result.push(proxy);
			}
			result
		}
	}

}

impl Deref for Vertex {
    type Target = RawVertex;

    fn deref<'a>(&'a self) -> &'a RawVertex {
        unsafe {
            &*(self.v)
        }
    }
}

impl DerefMut for Vertex {
    fn deref_mut<'a>(&'a mut self) -> &'a mut RawVertex {
        unsafe {
            &mut *(self.v)
        }
    }
}
