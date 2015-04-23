use std::collections::HashMap;
use std::mem;

use vertex::{Vertex,VertexProxy};
use traversal::{GraphQuery, GraphElement};

#[derive(Debug)]
pub struct Graph {
    elements: i64,
    // we transmute our Boxed vertex into a pointer later
    vertices: HashMap<i64, *mut Vertex>,
}

impl Graph {
    pub fn new() -> Box<Graph> {
        let vertices: HashMap<i64, *mut Vertex> = HashMap::new();
        Box::new(Graph{elements:0, vertices:vertices})
    }

    pub fn add_vertex(&mut self) -> VertexProxy {
        let new_id = self.elements + 1;
        self.elements += 1;
        let v = Vertex::new(new_id);
        let ptr: *mut Vertex = unsafe { mem::transmute(v) };

        self.vertices.insert(new_id, ptr);

        // return the proxy which knows it's own id
        VertexProxy{id:new_id, v: ptr}
    }

    pub fn get(&self, vertex_id:i64) -> Option<VertexProxy>  {
        let vertex_pointer = self.vertices.get(&vertex_id);
        // return a proxy
        match vertex_pointer {
            Some(ptr) =>
                Some(VertexProxy{id:vertex_id, v:*ptr}),
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

trait Traversable {
	fn inV(&self) -> Vec<VertexProxy>;
	fn outV(&self) -> Vec<VertexProxy>;

}
