use std::ops::{Deref, DerefMut};
use std::mem;
use std::collections::HashMap;
use property::Property;

use std::rc::Rc;
use edge::{EdgePointer, Edge};
use graph::TraversableToVertex;
use std::sync::RwLock;

/*
* storing in & out edges seperately should cut down on the number of "things" i have to traverse
* if a vertex has 10k edges (5k in and out 5k out) then doing something like*   g(v).outV()
* should be ok
*/
#[derive(Debug, Clone)]
pub struct RawVertex {
    pub id: i64,
    pub properties: HashMap<String, Property>,
    // pointers on both sides, yay
    pub out_edges: Vec<EdgePointer>,
    pub in_edges:  Vec<EdgePointer>,
}

pub type VertexPointer = Rc<Box<RwLock<RawVertex>>>;

impl RawVertex {
    pub fn new(id: i64) -> VertexPointer  {
        let mut props  = HashMap::new();

        let vertex = RawVertex{id:id,
                        properties: props,
                        out_edges: Vec::new(),
                        in_edges: Vec::new()};

        Rc::new(Box::new(RwLock::new(vertex)))
    }
}


#[derive(Clone, Debug)]
pub struct Vertex {
    pub id: i64,
    pub v: VertexPointer
}


impl Vertex {

	pub fn new(id: i64) -> Vertex {
        let raw = RawVertex::new(id);
        Vertex{id:id, v:raw}
	}

    pub fn add_edge(&mut self, to_vertex: &mut Vertex, label: &str) -> EdgePointer {
        println!("add_edge() with label {}", label);

        // let in_vertex =  &mut *(self.v);
        // let out_vertex = &mut *(to_vertex.v);

        // why do i need the raw shit?
        // let mut in_vertex = self;
        // let mut out_vertex = to_vertex;
        // create the edge
        println!("adding vertex of edge {}", label.to_string());

        // keep it on the heap but manage it myself
        let edge = Edge::new(self.clone(),
                             to_vertex.clone(),
                             label.to_string());

        self.write().unwrap().out_edges.push(edge.clone());
        // self.out_edges.push(edge.clone());

        to_vertex.write().unwrap().in_edges.push(edge.clone());
        edge
    }

    // TODO switch to accepting a &str
    pub fn set_property(&mut self, field: &str, value: Property) {
        self.write().unwrap().properties.insert(field.to_string(), value);
    }



    pub fn get_property(&self, field:&str) -> Option<Property> {
        self.write().unwrap().properties.get(&field.to_string()).cloned()
    }

    // TODO: slice?  iterator?
	pub fn out_edges(&self) -> Vec<EdgePointer> {
		let mut result = Vec::new();
		for edge in self.read().unwrap().out_edges.iter() {
			result.push(edge.clone());
		}
		result
	}

	pub fn in_edges(&self) -> Vec<EdgePointer> {
		let mut result = Vec::new();
		for edge in self.read().unwrap().in_edges.iter() {
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
	fn outV(&self, labels: &[&str]) -> Vec<Vertex> {
		let mut result = Vec::new();

        // convert our labels to a vector of strings
        let mut labels_as_strings : Vec<String> = Vec::new();

        for l in labels {
            let s = l.to_string();
            labels_as_strings.push(l.to_string());
        }

		for edge in self.read().unwrap().out_edges.iter() {
            let e = edge.read().unwrap();
            if labels.is_empty() || labels_as_strings.contains(&e.label) {
				result.push(edge.read().unwrap().to_vertex.clone());
            }
		}
		result
	}
	fn inV(&self) -> Vec<Vertex> {
		let mut result = Vec::new();
		for edge in self.read().unwrap().in_edges.iter() {
			result.push(edge.read().unwrap().from_vertex.clone());
		}
		result
	}

}

impl Deref for Vertex {
    type Target = VertexPointer;

    fn deref<'a>(&'a self) -> &'a VertexPointer {
        &self.v
    }
}

impl DerefMut for Vertex {
    fn deref_mut<'a>(&'a mut self) -> &'a mut VertexPointer {
        &mut self.v
    }
}
