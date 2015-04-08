
use vertex::Vertex;
use edge::Edge;
// define a traversal as a series of stages

struct VertexVisit {
    vertex: VertexProxy;
}

struct EdgeVisit {
    edge: EdgeProxy
}

enum TraversalStage {

}

struct Traversal {
    steps: Vec<TraversalStage>
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
    fn new(vertex: VertexProxy) -> Path {
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
            
            path.push( )
        }
        GraphQuery{paths}
    }
}

