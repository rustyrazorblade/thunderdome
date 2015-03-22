use std::ops::Deref;
use std::mem;

/*
* storing in & out edges seperately should cut down on the number of "things" i have to traverse
* if a vertex has 10k edges (5k in and out 5k out) then doing something like*   g(v).outV()
* should be ok
*/
#[derive(Debug)]
pub struct Vertex {
    pub id: i64,
    // pointers on both sides, yay
    pub out_edges: Vec<*mut Edge>,
    pub in_edges:  Vec<*mut Edge>,
}

impl Vertex {
    pub fn new(id: i64) -> Box<Vertex> {
        Box::new(Vertex{id:id, 
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

        let edge: *mut Edge = unsafe { mem::transmute(e) };

        in_vertex.out_edges.push(edge);
        out_vertex.in_edges.push(edge);
        EdgeProxy(edge)
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

#[derive(Debug)]
pub struct Edge {
    pub from_vertex: *mut Vertex,
    pub to_vertex: *mut Vertex
}

pub struct EdgeProxy(*mut Edge);
