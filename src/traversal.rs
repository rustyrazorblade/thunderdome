
use vertex::{RawVertex, Vertex};
use edge::{Edge, EdgeProxy};
use graph::TraversableToVertex;
use path::{Path, Element};
// define a traversal as a series of stages

pub struct GraphQuery {
	// many paths
    pub paths: Vec<Path>
}

impl GraphQuery {
    pub fn new(vertices: Vec<Vertex>) -> GraphQuery {
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
	pub fn push(&mut self, path:Path) {
		self.paths.push(path)
	}

	/*
	generic mapper for our queries
	future this may use worker threads to perform
	traversals
	 */
    fn map<F: Fn(&Path) -> Vec<Path>>(&self, closure: F) -> GraphQuery  {
        let mut result = GraphQuery::empty(); // result
		for x in self.paths.iter() {
			let mut tmp = closure(x);
			result.paths.append(&mut tmp)
		}
		result
    }

    pub fn outV(&self) -> GraphQuery {
		let f = |path: &Path| {
			println!("applying outV");
			let mut result : Vec<Path> = Vec::new();
			//take the final element in the path
			let element = path.last().unwrap();

			match element {
				&Element::RawVertex(ref v) => {
					println!("matched vertex");
					let mut tmp = v.outV();
					for vertex in tmp {
						println!("adding path based on vertex match");
						let new_path = path.clone();
						result.push(new_path);
					}
				},
				&Element::Edge(ref e) => {

				}
			}
			// apply outV

			result
		};
		self.map(f)
    }

//	pub fn inV(&self) -> GraphQuery {
//		let g = GraphQuery::empty();
//		let f = |element: &Element| {
//			element.inV()
//		};
//		self.map(f)
//	}
}
