use path::Element;
use vertex::Vertex;
use graph::Graph;

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
    pub fn add_child(&mut self, element: Element) {

    }
    pub fn child_count(&self) -> usize {
        match self.children {
            None => 0,
            Some(ref x) => x.len()
        }
    }



    // pub fn from_vertices(vlist: &[Vertex]) -> TreePath {
    //
    // }
}

#[test]
fn test_create_tree() {

    let mut g = Graph::new();
    let v = g.add_vertex();
    let v2 = g.add_vertex();
    let v3 = g.add_vertex();

    let mut t = TreePath::from_vertex(v);
    t.add_child(Element::Vertex(v2));
    t.add_child(Element::Vertex(v3));

    assert_eq!(2, t.child_count());

}
