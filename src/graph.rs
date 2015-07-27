use std::collections::HashMap;

use vertex::Vertex;
use path::Element;
use treepath::TreePath;
use parser::{parse, Arg};
use std::sync::mpsc::{channel, Sender};
use edge::{EdgePointer, Edge};


#[derive(Debug)]
pub struct Graph {
    elements: i64,
    vertices: HashMap<i64, Vertex>
}

// comes in through the request channel
pub struct Request {
    query: String
}


// graph is not thread safe - needs to be wrapped in an Arc<Mutex> when running as server
impl Graph {
    pub fn new() -> Box<Graph> {
        Box::new(Graph{elements:0,
                        vertices:HashMap::new() })
    }

    pub fn execution_channel(&self) -> Sender<Request> {
        let (tx, rx) = channel();
        // fire off listener thread for receiving messages

        tx
    }

    pub fn add_edge(&self, v1: &Vertex, v2: &Vertex, label: String) -> EdgePointer {
        println!("adding vertex of edge {}", label.to_string());

        // keep it on the heap but manage it myself
        let edge = Edge::new(v1.clone(), v2.clone(), label);

        v1.write().unwrap().out_edges.push(edge.clone());
        v2.write().unwrap().in_edges.push(edge.clone());
        edge

    }
    pub fn execute(&self, query: &str) -> Result<GraphQueryResult, &str>  {
        // parse the graph query

        // let mut steps_table : HashMap<&'static str, FnMut() -> Result<&'static str, &'static str> > = HashMap::new();
        // steps_table.insert("outV", traverse_out);
        // steps_table.insert("inV", traverse_in);

        // todo - validate query is using legit steps
        let mut result = GraphQueryResult::new();
        println!("===== Begin query {} ======", query);
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
                        "v" => self.vertex_query(&mut result, &step.args),
                        "V" => self.global_query(&result, &step.args),
                        _ => return Err("no step found")
                    };
                    match step_result {
                        Err(_) => return Err("fail"),
                        Ok(_) => ()
                    };
                    // execute function, passing step args
                }
            },
            Err(_) => {
                println!("SUCH FAIL!!!!")

            }
                // Result::Err("meh")
        };
        println!("===== query finished =====");
        Ok(result)
    }

    fn vertex_query(&self, result: &mut GraphQueryResult, args: &Vec<Arg>) ->
                    Result<&'static str, &'static str>  {

        println!("vertex query");
        // gather the requested vertices
        let mut error = false;

        for arg in args.iter() {
            // this better be an integer
            match *arg {
                Arg::Integer(x) => {
                    // lookup the vertex
                    let v = self.get(x);
                    match v {
                        Some(vertex) => {
                            result.tree.add_child(Element::Vertex(vertex));
                        },
                        None => { }
                    }
                }
                _ => error = true
            }
        }
        if error {
            Err("errors")
        } else {
            Ok("you are a pirate")
        }
    }

    fn global_query(&self, result: &GraphQueryResult, args: &Vec<Arg>)
        -> Result<&'static str, &'static str>  {
        println!("global query");
        unimplemented!();
        Ok("cool")
    }

    fn traverse_out_vertex(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("outV traversal");


        Ok("cool")
    }

    fn traverse_in_vertex(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("inV traversal");

        unimplemented!();
        Ok("cool")
    }

    fn traverse_out_edge(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("outE traversal");
        unimplemented!();
        Ok("cool")
    }

    fn traverse_in_edge(&self, args: &Vec<Arg>) -> Result<&'static str, &'static str>  {
        println!("inE traversal");
        unimplemented!();
        Ok("cool")
    }
    pub fn add_vertex(&mut self) -> Vertex {
        let new_id = self.elements + 1;
        self.elements += 1;
        let v = Vertex::new(new_id);
        self.vertices.insert(new_id, v.clone());
        // return the proxy which knows it's own id
        v
    }

    pub fn get(&self, vertex_id:i64) -> Option<Vertex>  {
        self.vertices.get(&vertex_id).cloned()
    }

}

pub trait TraversableToVertex {
	fn inV(&self) -> Vec<Vertex>;
	fn outV(&self, &[&str]) -> Vec<Vertex>;
//	fn inE(&self) -> Vec<Edge>;

}


pub struct GraphQueryResult {
    pub tree: TreePath
}

impl GraphQueryResult {
    pub fn new() -> GraphQueryResult {
        GraphQueryResult{ tree: TreePath::new() }
    }
}
