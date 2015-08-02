use std::ops::{Deref, DerefMut};
use std::mem;
use std::collections::HashMap;

use std::rc::Rc;
use edge::{EdgePointer, Edge};
use graph::TraversableToVertex;
use std::sync::RwLock;
use std::any::Any;

// this is what we want to expose to the outside world
pub type VertexPointer = Rc<RwLock<Vertex>>;

#[derive(Debug)]
pub struct Vertex {
    pub id: i64,
    pub properties: HashMap<String, Box<Any>>,
    pub out_edges: Vec<EdgePointer>,
    pub in_edges:  Vec<EdgePointer>,

}

impl Clone for Vertex {
    fn clone(&self) -> Self {
        // Clone does not clone the edges
        // cloning is only useful when building subgraphs
        // we can't keep the edges because then it's a recursive mess
        Vertex{id:self.id,
             properties:HashMap::new(),
             out_edges: Vec::new(),
             in_edges: Vec::new() }
    }

    fn clone_from(&mut self, source: &Self) {
        // todo maybe?
    }
}

impl Vertex {

	pub fn new(id: i64) -> VertexPointer {

        let vertex = Vertex{id:id,
                            properties: HashMap::new(),
                            out_edges: Vec::new(),
                            in_edges: Vec::new()};

        Rc::new(RwLock::new(vertex))
	}

    // TODO switch to accepting a &str
    // pub fn set_property(&mut self, field: &str, value: Property) {
    //     self.properties.insert(field.to_string(), value);
    // }
    //
    //
    //
    // pub fn get_property(&self, field:&str) -> Option<Property> {
    //     self.properties.get(&field.to_string()).cloned()
    // }

}

impl TraversableToVertex for Vertex {
	/* basic traversal operations
	   cannot be chained
	   look at traversal.rs for breadth first chaining
	*/

	/* returns all the outV vertex proxies
	   mainly for internal use
	*/
	fn outV(&self) -> Vec<VertexPointer> {
        self.out_edges.iter().map(|x|
            x.read().unwrap().from_vertex.clone()
            ).collect()
	}

	fn inV(&self) -> Vec<VertexPointer> {
        self.in_edges.iter().map(|x|
            x.read().unwrap().from_vertex.clone()
            ).collect()
	}

}
