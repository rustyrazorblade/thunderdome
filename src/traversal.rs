
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

pub enum GraphIterable {
    Vertex(VertexProxy),
    Edge(EdgeProxy)
}



// used to track a single graph traversal
// will be used in conjunction with a GraphQuery

struct Path {
    path: Vec<GraphIterable>
}

impl Path {
    fn new(vertex: VertexProxy)  {
    }
}

pub struct GraphQuery {
	// many paths
    paths: Vec<Path>
}

impl GraphQuery {
    pub fn new(vertices: Vec<VertexProxy>) -> GraphQuery {
        // create a new path for each proxy, add the proxy
        let paths = Vec::new();
        let path: Path;
        for p in vertices {

            //paths.push(path)
        }
        GraphQuery{paths:paths}
    }

    pub fn empty() -> GraphQuery {
        // return an empty graph query
        GraphQuery{paths:Vec::new()}
    }

    fn map(closure: &Fn() -> Path) -> GraphQuery  {
        GraphQuery{paths:Vec::new()}

    }
    pub fn outV(&self) -> GraphQuery {
        let g = GraphQuery::empty();
        g
    }
}

