use std::ops::{Deref, DerefMut};
use std::mem;
use std::collections::HashMap;
use property::Property;

use std::rc::Rc;
use edge::{RawEdge,Edge};
use graph::TraversableToVertex;

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
    pub out_edges: Vec<Edge>,
    pub in_edges:  Vec<Edge>,
}

impl RawVertex {
    pub fn new(id: i64) -> Rc<Box<RawVertex>> {
        let mut props  = HashMap::new();
        Rc::new(Box::new(RawVertex{id:id,
                        properties: props,
                        out_edges: Vec::new(),
                        in_edges: Vec::new()}))
    }
}


#[derive(Clone, Debug)]
pub struct Vertex {
    pub id: i64,
    pub v: Rc<Box<RawVertex>>,
}

impl Vertex {

	pub fn new(id: i64) -> Vertex {
        let raw = RawVertex::new(id);
        Vertex{id:raw.id, v:raw}
	}

    pub fn add_edge(&mut self, to_vertex: &mut Vertex, label: &str) -> Edge {
        println!("add_edge() with label {}", label);

        // let in_vertex =  &mut *(self.v);
        // let out_vertex = &mut *(to_vertex.v);

        // why do i need the raw shit?
        let in_vertex = self;
        let out_vertex = to_vertex;
        // create the edge
        println!("adding vertex of edge {}", label.to_string());

        let e = Box::new(RawEdge{from_vertex: self.clone(),
                                 to_vertex: to_vertex.clone(),
                                 label: label.to_string() });

        // keep it on the heap but manage it myself
        let edge = Edge::new(in_vertex.clone(),
                             out_vertex.clone(),
                             label.to_string());

        in_vertex.out_edges.push(edge.clone());
        out_vertex.in_edges.push(edge.clone());
        edge
    }

    // TODO switch to accepting a &str
    pub fn set_property(&mut self, field: &str, value: Property) {
        self.properties.insert(field.to_string(), value);
    }



    pub fn get_property(&self, field:&str) -> Option<&Property> {
        self.properties.get(&field.to_string())
    }

	pub fn outE(&self) -> Vec<Edge> {
		let mut result = Vec::new();
		for &edge in self.out_edges.iter() {
			result.push(edge.clone());
		}
		result
	}

	pub fn inE(&self) -> Vec<Edge> {
		let mut result = Vec::new();
		for &edge in self.in_edges.iter() {
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

		for &edge in self.out_edges.iter() {
            if labels.is_empty() || labels_as_strings.contains(&edge.label) {
				result.push(edge.to_vertex.clone());
            }
		}
		result
	}
	fn inV(&self) -> Vec<Vertex> {
		let mut result = Vec::new();
		for &edge in self.in_edges.iter() {
			result.push(edge.from_vertex.clone());
		}
		result
	}

}

impl Deref for Vertex {
    type Target = Rc<Box<RawVertex>>;

    fn deref<'a>(&'a self) -> &'a Rc<Box<RawVertex>> {
        self.v
    }
}

impl DerefMut for Vertex {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Rc<Box<RawVertex>> {
        self.v
    }
}
