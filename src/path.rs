use vertex::{RawVertex, Vertex};
use edge::{RawEdge, Edge};
use graph::TraversableToVertex;
// used to track a single graph traversal
// will be used in conjunction with a GraphQuery

#[derive(Clone)]
pub enum Element {
    Vertex(Vertex),
    Edge(Edge)
}


pub struct Path {
    // TODO make this private
    pub path: Vec<Element>
}

impl Path {
    pub fn new(vertex: Vertex) -> Path {
		Path{path:vec![Element::Vertex(vertex)]}
    }
	pub fn new_empty() -> Path {
		Path{path:vec![]}
	}

	/*
	takes the current path and a vector of elements
	and returns a vector of new paths,
	each of which extends the existing path
	 */
    // TODO return an Option
	pub fn permute(&self, elements_to_add: &[Element]) -> Vec<Path> {
		let mut result: Vec<Path> = Vec::new();
		for element in elements_to_add.iter() {
			// construct a new path
			let mut p = self.clone();
			p.path.push(element.clone());
			result.push(p);
		}
		result
	}
    pub fn last(&self) -> Option<&Element> {
        self.path.last()
    }

}

impl Clone for Path {
	fn clone(&self) -> Path {
		Path::new_empty()
	}

	fn clone_from(&mut self, source: &Self ) {

	}
}
