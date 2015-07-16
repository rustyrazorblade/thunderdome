use std::collections::HashMap;
use std::mem;

use vertex::{RawVertex,Vertex};
use traversal::GraphQuery;
use path::Element;
use treepath::TreePath;
use edge::Edge;
use parser::{parse, Arg};

#[derive(Debug)]
pub struct Graph {
    elements: i64,
    // we transmute our Boxed vertex into a pointer later
    vertices: HashMap<i64, *mut RawVertex>,
    queue: Vec<String>,
}

// graph is not thread safe - needs to be wrapped in an Arc<Mutex> when running as server
impl Graph {
    pub fn new() -> Box<Graph> {
        let vertices: HashMap<i64, *mut RawVertex> = HashMap::new();
        Box::new(Graph{elements:0, vertices:vertices, queue:Vec::with_capacity(100)})
    }

    // todo actually return a result DERP DERP DERP
    pub fn execute(&self, query: &str) {
        // parse the graph query

        // let mut steps_table : HashMap<&'static str, FnMut() -> Result<&'static str, &'static str> > = HashMap::new();
        // steps_table.insert("outV", traverse_out);
        // steps_table.insert("inV", traverse_in);

        // todo - validate query is using legit steps

        match parse(query) {
            Ok(query) => {
                // execute each of the steps sequentially
                for step in query.steps.iter() {
                    // lookup step in hash table
                    println!("{}", step);

                    let name : &str = &step.name;

                    // there has to be a better way of doing this?
                    let step_result = match name {
                        "outV" => self.traverse_out_vertex(&step.args),
                        "inV" => self.traverse_in_vertex(&step.args),
                        "outE" => self.traverse_out_edge(&step.args),
                        "inE" => self.traverse_in_edge(&step.args),
                        "v" => self.vertex_query(&step.args),
                        "V" => self.global_query(&step.args),
                        _ => Err("no thingy found")
                    };
                    // execute function, passing step args
                }
            },
            Err(x) => {
                println!("SUCH FAIL!!!!")
            }
                // Result::Err("meh")
        }
        println!("query finished");
        ()
    }

    fn vertex_query(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("vertex query");
        // gather the requested vertices
        Ok("cool")
    }

    fn global_query(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("global query");
        Ok("cool")
    }

    fn traverse_out_vertex(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("outV traversal");

        Ok("cool")
    }

    fn traverse_in_vertex(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("inV traversal");

        Ok("cool")
    }

    fn traverse_out_edge(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("outE traversal");
        Ok("cool")
    }

    fn traverse_in_edge(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("inE traversal");
        Ok("cool")
    }
    pub fn add_vertex(&mut self) -> Vertex {
        let new_id = self.elements + 1;
        self.elements += 1;
        let v = RawVertex::new(new_id);
        let ptr: *mut RawVertex = unsafe { mem::transmute(v) };

        self.vertices.insert(new_id, ptr);

        // return the proxy which knows it's own id
        Vertex{id:new_id, v: ptr}
    }

    pub fn get(&self, vertex_id:i64) -> Option<Vertex>  {
        let vertex_pointer = self.vertices.get(&vertex_id);
        // return a proxy
        match vertex_pointer {
            Some(ptr) =>
                Some(Vertex{id:vertex_id, v:*ptr}),
            None =>
                None
        }
    }

    // this will be used to start a graph query
    pub fn v(&self, vertex_id:i64) -> GraphQuery {
        // grab the original vertex
        let vertex = self.get(vertex_id);
        let mut v = Vec::new();
        match vertex {
            Some(result) => {
                v.push(result);
                GraphQuery::new(v)
            }
            None => GraphQuery::empty()
        }
    }
}

pub trait TraversableToVertex {
	fn inV(&self) -> Vec<Vertex>;
	fn outV(&self, &[&str]) -> Vec<Vertex>;
//	fn inE(&self) -> Vec<Edge>;


}


struct GraphQueryResult {
    results: TreePath
}
