//! Module docs?  whatever


use path::Element;
use vertex::Vertex;
use graph::Graph;
use std::collections::HashMap;

#[derive(Debug, Display, Clone)]
pub struct TreePath {
    pub element: Element,
    pub children: Box<HashMap<usize, TreePath>>,
    pub element_count: usize
}

impl TreePath {
    // we need to be able to use this on new graph query results because
    // we don't always start with a single vertex
    pub fn new() -> TreePath {

        TreePath{element:Element::TreeRoot, children:Box::new(HashMap::new()), element_count:0}
    }

    // convenience method
    pub fn from_vertex(vertex: Vertex) -> TreePath {
        let e = Element::Vertex(vertex);
        TreePath::from_element(e)
    }

    pub fn from_element(element: Element) -> TreePath {
        TreePath{element:element, children:Box::new(HashMap::new()), element_count:0}
    }

    pub fn add_child(&mut self, element: Element) {
        let child = TreePath::from_element(element);
                // create a new Treepath
        self.children.insert(self.element_count, child);
        self.element_count += 1;
    }

    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    // iterator?
    pub fn leaves(&self)  {

    }
    // pub fn from_vertices(vlist: &[Vertex]) -> TreePath {
    //
    // }
}
