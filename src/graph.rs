use std::collections::HashMap;
use std::mem;

use vertex::{RawVertex,Vertex};
use traversal::{GraphQuery, GraphElement};
use edge::EdgeProxy;

#[derive(Debug)]
pub struct Graph {
    elements: i64,
    // we transmute our Boxed vertex into a pointer later
    vertices: HashMap<i64, *mut RawVertex>,
}

impl Graph {
    pub fn new() -> Box<Graph> {
        let vertices: HashMap<i64, *mut RawVertex> = HashMap::new();
        Box::new(Graph{elements:0, vertices:vertices})
    }

    pub fn add_vertex(&mut self) -> Vertex {
        let new_id = self.elements + 1;
        self.elements += 1;
        let v = RawVertex::new(new_id);
        let ptr: *mut RawVertex = unsafe { mem::transmute(v) };

        self.vertices.insert(new_id, ptr);

        // return the proxy which knows it's own id
        Vertex{id:new_id, v: ptr}
    }

    pub fn get(&self, vertex_id:i64) -> Option<Vertex>  {
        let vertex_pointer = self.vertices.get(&vertex_id);
        // return a proxy
        match vertex_pointer {
            Some(ptr) =>
                Some(Vertex{id:vertex_id, v:*ptr}),
            None =>
                None
        }
    }

    // this will be used to start a graph query
    pub fn v(&self, vertex_id:i64) -> GraphQuery {
        // grab the original vertex
        let vertex = self.get(vertex_id);
        let mut v = Vec::new();
        match vertex {
            Some(result) => {
                v.push(result);
                GraphQuery::new(v)
            }
            None => GraphQuery::empty()
        }
    }
}

pub trait TraversableToVertex {
	fn inV(&self) -> Vec<Vertex>;
	fn outV(&self) -> Vec<Vertex>;
//	fn inE(&self) -> Vec<EdgeProxy>;


}
