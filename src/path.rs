use vertex::Vertex;
use edge::Edge;
use graph::TraversableToVertex;
// used to track a single graph traversal
// will be used in conjunction with a GraphQuery

#[derive(Clone, Debug)]
pub enum Element {
    Vertex(Vertex),
    Edge(Edge),
    TreeRoot,
}
