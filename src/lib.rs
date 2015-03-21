
use std::ops::Deref;

pub mod graph {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Graph {
        elements: i64,
        vertices: HashMap<i64, Box<Vertex>>,
    }

    impl Graph {
        pub fn new() -> Box<Graph> {
            let vertices: HashMap<i64, Box<Vertex>> = HashMap::new();
            Box::new(Graph{elements:0, vertices:vertices})
        }

        pub fn add_vertex(&mut self) -> VertexProxy {
            let new_id = self.elements + 1;
            self.elements += 1;
            let mut v = Vertex::new(new_id);

            let ptr: *mut Box<Vertex> = &mut v as *mut Box<Vertex>;

            self.vertices.insert(new_id, v);

            // return the proxy which knows it's own id
            VertexProxy{id:new_id, v: ptr}
        }
    }

    /*
    I'm not sure on this, but it feels reasonable.  a vertex can own it's out_edges.
    this should? make it easier to manage the memory?

    additionally, it should cut down on the number of "things" i have to traverse
    if a vertex has 10k edges (5k in and out 5k out) then doing something like

    g(v).outV()

    should be faster to iterate over

    the cost is a little code complexity but i think that won't be too bad
     */
    #[derive(Debug)]
    struct Vertex {
        id: i64,
        out_edges: Vec<Edge>,
        in_edges: *mut Vec<Edge>,
    }

    impl Vertex {
        pub fn new(id: i64) -> Box<Vertex> {

            let mut edges : Vec<Edge> = Vec::new();
            let edges_ptr : *mut Vec<Edge> = &mut edges;
            let out_edges: Vec<Edge> = Vec::new();
            let vertex = Vertex{id:id, out_edges: out_edges, in_edges:edges_ptr};
            let mut v = Box::new(vertex);
            return v;
        }

        pub fn add_out_edge(&mut self, edge: Edge ) {
            self.out_edges.push(edge)
        }

        pub fn add_in_edge() {

        }

    }

    #[derive(Debug)]
    pub struct VertexProxy {
        pub id: i64,
        v: *mut Box<Vertex>,
    }

    //let i: u32 = 1;
    //// explicit cast
    //let p_imm: *const u32 = &i as *const u32;
    //let mut m: u32 = 2;
    //// implicit coercion
    //let p_mut: *mut u32 = &mut m;
    //
    //unsafe {
    //let ref_imm: &u32 = &*p_imm;
    //let ref_mut: &mut u32 = &mut *p_mut;
    //}

    impl VertexProxy {
        pub fn add_edge(&mut self, to_vertex: &mut VertexProxy) {
            let in_vertex: &mut Box<Vertex>;
            let out_vertex: &mut Box<Vertex>;

            unsafe {
                in_vertex =  &mut *(self.v);
                out_vertex = &mut *(to_vertex.v);
            }

            // create the edge
            let mut e = Edge{from_vertex: self.v, to_vertex: to_vertex.v};
            in_vertex.add_out_edge(e);
        }
        pub fn query(self) {

        }
    }


    /*
    impl<T> Deref for DerefExample<T> {
        type Target = T;

        fn deref<'a>(&'a self) -> &'a T {
            &self.value
        }
    }
     */

    #[derive(Debug)]
    struct Edge {
        from_vertex: *const Box<Vertex>,
        to_vertex: *const Box<Vertex>
    }
}

mod tests {
    use graph::Graph;

    #[test]
    fn test_unsafe_vertex() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();
        assert!(v1.id == 1);

        let v2 = g.add_vertex();
        assert!(v2.id == 2);
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new();
        let mut v1 = g.add_vertex();
        let mut v2 = g.add_vertex();

        v1.add_edge(&mut v2);
    }
}
