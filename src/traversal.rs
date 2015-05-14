
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
    fn map<F: Fn(&Path) -> Option<&[Element]>>(&self, closure: F) -> GraphQuery  {
        let mut result = GraphQuery::empty(); // result
		for path in self.paths.iter() {
			let mut tmp = closure(path);
            // currently gets back a Vec<Path> but what if it gets Elements?
            match tmp {
                Some(x) => {
                    let new_elements = path.permute(x);
                    result.paths.push_all(&new_elements);
                },
                None =>
                    {}
            };
		}
		result
    }

    pub fn outV(&self) -> GraphQuery {
        // needs to be modified to return a Vec of elements or a slice?
        // map can figure out the rest
		let f = |path: &Path| {
			println!("applying outV");
			//take the final element in the path
			let element = path.last().unwrap();

			let result = match element {
				&Element::RawVertex(ref v) => {
					Some(v.outV())
				},
				&Element::Edge(ref e) => {
                    None
				}
			};
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
