//! Module docs?  whatever


use path::Element;
use vertex::Vertex;
use graph::Graph;

#[derive(Debug, Display)]
pub struct TreePath {
    pub element: Element,
    pub children: Option<Box<Vec<TreePath>>>
}

impl TreePath {
    // we need to be able to use this on new graph query results because
    // we don't always start with a single vertex
    pub fn new() -> TreePath {
        TreePath{element:Element::TreeRoot, children:None}
    }
    // convenience method
    pub fn from_vertex(vertex: Vertex) -> TreePath {
        let e = Element::Vertex(vertex);
        TreePath::from_element(e)
    }

    pub fn from_element(element: Element) -> TreePath {
        TreePath{element:element, children:None}
    }

    pub fn add_child(&mut self, element: Element) {
        let child = TreePath::from_element(element);
        match self.children {
            None => {
                let mut v = Box::new(Vec::new());
                // create a new Treepath
                v.push(child);
                self.children = Some(v);
            }
            Some(ref mut children) => {
                children.push(child);
            }
        }
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