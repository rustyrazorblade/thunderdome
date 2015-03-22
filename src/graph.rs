use std::collections::HashMap;
use std::mem;
use vertex::{Vertex,VertexProxy};

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

    pub fn get(vertex_id:i64)  {
        
    }
}
