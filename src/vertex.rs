use std::ops::{Deref, DerefMut};
use std::mem;
use std::collections::HashMap;
use property::PropertyMap;

use std::rc::Rc;
use edge::{EdgePointer, Edge};
use graph::TraversableToVertex;
use std::sync::RwLock;

use std::any::Any;

// this is what we want to expose to the outside world
pub type VertexPointer = Rc<RwLock<Vertex>>;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub id: i64,
    pub properties: PropertyMap,
    // pointers on both sides, yay
    pub out_edges: Vec<EdgePointer>,
    pub in_edges:  Vec<EdgePointer>,

}



impl Vertex {

	pub fn new(id: i64) -> VertexPointer {

        let vertex = Vertex{id:id,
                        properties: PropertyMap::new(),
                        out_edges: Vec::new(),
                        in_edges: Vec::new()};

        Rc::new(RwLock::new(vertex))
	}

    pub fn set(&mut self, field: String, value: Box<Any> ) {

    }
    pub fn get(&mut self, field:String) {

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
