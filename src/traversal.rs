
use vertex::{RawVertex, Vertex};
use edge::{RawEdge, Edge};
use graph::TraversableToVertex;
use path::{Path, Element};
use std::iter::Extend;

// use std::iter::Iterator;

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
    fn map<F: Fn(&Path) -> Vec<Element>>(&self, closure: F) -> GraphQuery  {
        let mut result = GraphQuery::empty(); // result
        for path in self.paths.iter() {
            let mut tmp = closure(&path);
            // let mut tmp2 = tmp.as_slice();
            // currently gets back a Vec<Path> but what if it gets Elements?
            let new_elements = path.permute(&tmp);
            result.paths.extend(new_elements);
        }
        result
    }

    pub fn outV(&self, labels: Vec<&str>) -> GraphQuery {
        // map can figure out the rest
        let f = |path: &Path| {
            //take the final element in the path
            let element = path.last().unwrap();

            let result : Vec<Element> = match element {
                &Element::Vertex(v) =>
                    v.outV(&["teest"]).iter().map(|v| Element::Vertex(*v)).collect(),
                    // Vec::new(),
                &Element::Edge(ref e) =>
                    Vec::new()
            };
            // apply outV
            result
        };
        self.map(f)
    }

//    pub fn inV(&self) -> GraphQuery {
//        let g = GraphQuery::empty();
//        let f = |element: &Element| {
//            element.inV()
//        };
//        self.map(f)
//    }
}
