

pub mod graph {
    use std::collections::HashMap;
    use std::ops::Deref;
    use std::mem;

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

            let mut v = Vertex::new(new_id);

            let ptr: *mut Vertex = unsafe { mem::transmute(v) };

            self.vertices.insert(new_id, ptr);

            // return the proxy which knows it's own id
            VertexProxy{id:new_id, v: ptr}
        }
        pub fn get(vertex_id:i64)  {
        }
    }

    /*
    * storing in & out edges seperately should cut down on the number of "things" i have to traverse
    * if a vertex has 10k edges (5k in and out 5k out) then doing something like
    *   g(v).outV()
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
        pub fn add_edge(&mut self, to_vertex: &mut VertexProxy) {
            let in_vertex: &mut Vertex;
            let out_vertex: &mut Vertex;

            unsafe {
                in_vertex =  &mut *(self.v);
                out_vertex = &mut *(to_vertex.v);
            }

            // create the edge
            let mut e = Box::new(Edge{from_vertex: self.v, to_vertex: to_vertex.v});

            let mut edge: *mut Edge = unsafe { mem::transmute(e) };

            in_vertex.out_edges.push(edge);
            out_vertex.in_edges.push(edge);
        }

        pub fn query(self) {

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
        from_vertex: *const Vertex,
        to_vertex: *const Vertex
    }

    struct EdgeProxy {
        edge: *mut Edge
    }

    impl Deref for EdgeProxy {
        type Target = Edge;   
        fn deref<'a>(&'a self) -> &'a Edge {
            unsafe { &*(self.edge) }
        }
    }

}

mod tests {
    use graph::{Graph, Vertex};
    
    #[test]
    fn test_unsafe_vertex() {
        let v = Vertex::new(1);

        let out_edges = &v.out_edges;
        let in_edges = &v.in_edges;
    }

    #[test]
    fn test_deref() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();
        let vertex = &*v1;
        assert!(vertex.id == 1);
    }
    
    #[test]
    fn test_new_vertex() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();

        assert_eq!(v1.id, 1);

        unsafe {
            let vertex = &*v1;
            assert_eq!((*v1).id, 1);
        }

        let v2 = g.add_vertex();
        assert!(v2.id == 2);
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new();
        let mut v1 = g.add_vertex();
        let mut v2 = g.add_vertex();

        v1.add_edge(&mut v2);
        v1.add_edge(&mut v2);
        v1.add_edge(&mut v2);

        assert_eq!(v1.out_edges.len(), v2.in_edges.len());


        // ensure the vertex and edge pointers are correct
    }
}
