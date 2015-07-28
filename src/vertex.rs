use std::ops::{Deref, DerefMut};
use std::mem;
use std::collections::HashMap;
use property::Property;

use std::rc::Rc;
use edge::{EdgePointer, Edge};
use graph::TraversableToVertex;
use std::sync::RwLock;


// this is what we want to expose to the outside world
pub type VertexPointer = Rc<RwLock<Vertex>>;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub id: i64,
    pub properties: HashMap<String, Property>,
    // pointers on both sides, yay
    pub out_edges: Vec<EdgePointer>,
    pub in_edges:  Vec<EdgePointer>,

}


impl Vertex {

	pub fn new(id: i64) -> VertexPointer {
        let mut props  = HashMap::new();

        let vertex = Vertex{id:id,
                        properties: props,
                        out_edges: Vec::new(),
                        in_edges: Vec::new()};

        Rc::new(RwLock::new(vertex))
	}


    // TODO switch to accepting a &str
    pub fn set_property(&mut self, field: &str, value: Property) {
        self.properties.insert(field.to_string(), value);
    }



    pub fn get_property(&self, field:&str) -> Option<Property> {
        self.properties.get(&field.to_string()).cloned()
    }

    // TODO: slice?  iterator?
	pub fn out_edges(&self) -> Vec<EdgePointer> {
		let mut result = Vec::new();
		for edge in self.out_edges.iter() {
			result.push(edge.clone());
		}
		result
	}

	pub fn in_edges(&self) -> Vec<EdgePointer> {
		let mut result = Vec::new();
		for edge in self.in_edges.iter() {
			result.push(edge.clone());
		}
		result
	}

}

impl TraversableToVertex for Vertex {
	/* basic traversal operations
	   cannot be chained
	   look at traversal.rs for breadth first chaining
	*/

	/* returns all the outV vertex proxies
	   mainly for internal use
	*/
	fn outV(&self, labels: &[&str]) -> Vec<VertexPointer> {
		let mut result = Vec::new();

        // convert our labels to a vector of strings
        let mut labels_as_strings : Vec<String> = Vec::new();

        for l in labels {
            let s = l.to_string();
            labels_as_strings.push(l.to_string());
        }

		for edge in self.out_edges.iter() {
            let e = edge.read().unwrap();
            if labels.is_empty() || labels_as_strings.contains(&e.label) {
				result.push(edge.read().unwrap().to_vertex.clone());
            }
		}
		result
	}
	fn inV(&self) -> Vec<VertexPointer> {
		let mut result = Vec::new();
		for edge in self.in_edges.iter() {
			result.push(edge.read().unwrap().from_vertex.clone());
		}
		result
	}

}
