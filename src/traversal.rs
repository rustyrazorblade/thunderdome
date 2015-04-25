
use vertex::{Vertex, VertexProxy};
use edge::{Edge, EdgeProxy};
use graph::TraversableToVertex;
// define a traversal as a series of stages

struct VertexVisit {
    vertex: VertexProxy
}

struct EdgeVisit {
    edge: EdgeProxy
}

enum TraversalStage {

}

pub enum GraphElement {
    Vertex(VertexProxy),
    Edge(EdgeProxy)
}



// used to track a single graph traversal
// will be used in conjunction with a GraphQuery

pub struct Path {
    path: Vec<GraphElement>
}

impl Path {
    fn new(vertex: VertexProxy) -> Path {
		Path{path:vec![GraphElement::Vertex(vertex)]}
    }
	fn new_empty() -> Path {
		Path{path:vec![]}
	}

}

impl Clone for Path {
	fn clone(&self) -> Path {
		Path::new_empty()
	}

	fn clone_from(&mut self, source: &Self ) {

	}
}

pub struct GraphQuery {
	// many paths
    pub paths: Vec<Path>
}

impl GraphQuery {
    pub fn new(vertices: Vec<VertexProxy>) -> GraphQuery {
        // create a new path for each proxy, add the proxy
        let mut paths = Vec::new();
        for p in vertices {
			let mut path = Path::new(p);
            paths.push(path)
        }
        GraphQuery{paths:paths}
    }

    pub fn empty() -> GraphQuery {
        // return an empty graph query
        GraphQuery{paths:Vec::new()}
    }

	pub fn get_paths(&self) -> &Vec<Path> {
		&self.paths
	}
	pub fn push(&self, path:Path) {
		self.paths.push(path)
	}

	/*
	generic mapper for our queries
	future this may use worker threads to perform
	traversals
	 */
    fn full_map<F: Fn(&Path) -> Vec<Path>>(&self, closure: F) -> GraphQuery  {
        let mut result = GraphQuery::empty(); // result
		for x in self.paths.iter() {
			let mut tmp = closure(x);
			result.paths.append(&mut tmp)
		}
		result
    }

	/*
	accepts a proxy and a closure, per
	takes the last element
	 */
	fn map<F: Fn(&GraphElement) -> Vec<GraphElement>>(&self, closure: F)
		-> GraphQuery {
		let mut result = GraphQuery::empty();
		for path in self.paths.iter() {
			// apply the closure to the last element in each map
			// if nothing is returned, we should not use the path anymore
			let element = path.path.last().unwrap();
			let mut tmp = closure(element);
			let new_path = path.clone();
			result.push(new_path);
		}

		result

	}

	/*

	 */
    pub fn outV(&self) -> GraphQuery {
		let f = |path: &Path| {
			println!("applying outV");
			let mut result : Vec<Path> = Vec::new();
			//take the final element in the path
			let element = path.path.last().unwrap();

			match element {
				&GraphElement::Vertex(ref v) => {
					println!("matched vertex");
					let mut tmp = v.outV();
					for vertex in tmp {
						println!("adding path based on vertex match");
						let new_path = path.clone();
						result.push(new_path);
					}
				},
				&GraphElement::Edge(ref e) => {

				}
			}
			// apply outV

			result
		};
		self.full_map(f)
    }

//	pub fn inV(&self) -> GraphQuery {
//		let g = GraphQuery::empty();
//		let f = |element: &GraphElement| {
//			element.inV()
//		};
//		self.map(f)
//	}
}

