
use vertex::{Vertex, VertexProxy};
use edge::{Edge, EdgeProxy};
// define a traversal as a series of stages

struct VertexVisit {
    vertex: VertexProxy
}

struct EdgeVisit {
    edge: EdgeProxy
}

enum TraversalStage {

}

enum GraphIterable {
    VertexProxy,
    EdgeProxy
}



// used to track a single graph traversal
//
struct Path {
    path: GraphIterable
}

impl Path {
    fn new(vertex: VertexProxy)  {
    }
}

struct GraphQuery {
    paths: Vec<GraphIterable>
}

impl GraphQuery {
    fn new(vertices: Vec<VertexProxy>) -> GraphQuery {
        // create a new path for each proxy, add the proxy
        let paths = Vec::new();
        let path: Path;
        for p in vertices {
            
            //paths.push(path)
        }
        GraphQuery{paths:paths}
    }

    
    fn map(closure: &Fn() -> i32) -> GraphQuery  {
        GraphQuery{paths:Vec::new()}

    }
}

