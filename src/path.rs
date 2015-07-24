use vertex::VertexPointer;
use edge::Edge;
// used to track a single graph traversal
// will be used in conjunction with a GraphQuery

#[derive(Clone, Debug)]
pub enum Element {
    Vertex(VertexPointer),
    Edge(Edge),
    TreeRoot,
}
