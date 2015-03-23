use std::ops::{Deref, DerefMut};
use std::mem;
use std::collections::HashMap;

#[derive(Debug)]
pub enum GraphProperty {
    Int(i64),
    Float(f64), 
    String(String),
}

use edge::{Edge,EdgeProxy};

/*
* storing in & out edges seperately should cut down on the number of "things" i have to traverse
* if a vertex has 10k edges (5k in and out 5k out) then doing something like*   g(v).outV()
* should be ok
*/
#[derive(Debug)]
pub struct Vertex {
    pub id: i64,
    pub properties: HashMap<String, GraphProperty>,
    // pointers on both sides, yay
    pub out_edges: Vec<*mut Edge>,
    pub in_edges:  Vec<*mut Edge>,
}

impl Vertex {
    pub fn new(id: i64) -> Box<Vertex> {
        let mut props  = HashMap::new();
        Box::new(Vertex{id:id, 
                        properties: props,
                        out_edges: Vec::new(), 
                        in_edges: Vec::new()})
    }

}

pub struct VertexProxy {
    pub id: i64,
    pub v: *mut Vertex,
}

impl VertexProxy {
    pub fn add_edge(&mut self, to_vertex: &mut VertexProxy) -> EdgeProxy {
        let in_vertex: &mut Vertex;
        let out_vertex: &mut Vertex;

        unsafe {
            in_vertex =  &mut *(self.v);
            out_vertex = &mut *(to_vertex.v);
        }

        // create the edge
        let e = Box::new(Edge{from_vertex: self.v, to_vertex: to_vertex.v});

        // keep it on the heap but manage it myself
        let edge: *mut Edge = unsafe { mem::transmute(e) };

        in_vertex.out_edges.push(edge);
        out_vertex.in_edges.push(edge);
        EdgeProxy{edge:edge}
    }
    
    pub fn set_property(&mut self, field: String, value: GraphProperty) {
        self.properties.insert(field, value);
    }


    pub fn query(self) {
        unimplemented!()
    }
}

impl Deref for VertexProxy {
    type Target = Vertex;

    fn deref<'a>(&'a self) -> &'a Vertex {
        unsafe {
            &*(self.v)
        }
    }
}

impl DerefMut for VertexProxy {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Vertex {
        unsafe {
            &mut *(self.v)
        }
    }
}

enum GraphIterable {
    VertexProxy,
    EdgeProxy
}

struct GraphPath {
    path: GraphIterable
}
