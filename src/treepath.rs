use path::Element;
use vertex::Vertex;

#[derive(Debug, Display)]
pub struct TreePath {
    pub element: Element,
    pub children: Option<Box<Vec<TreePath>>>
}

impl TreePath {
    pub fn from_vertex(vertex: Vertex) -> TreePath {
        let e = Element::Vertex(vertex);
        TreePath{element:e, children:None}
    }
    // pub fn from_vertices(vlist: &[Vertex]) -> TreePath {
    //
    // }
}
