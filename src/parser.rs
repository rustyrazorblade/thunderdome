#![plugin(peg_syntax_ext)]
use std::fmt;
use std::collections::HashMap;


peg_file! gremlin("gremlin.rustpeg");

pub fn parse(g: &str) -> Result<ParsedGraphQuery, gremlin::ParseError> {
    let parsed = pre_parse(g);
    // verify all the steps actually make sense
    // is it a query to a single vertex or a global query?
    parsed
}

pub fn pre_parse(g: &str) -> Result<ParsedGraphQuery, gremlin::ParseError> {
    gremlin::query(g)
}



/*
returned from the peg parser
we'll need to take each of the steps
and use them to construct an actual GraphQuery
*/
pub struct ParsedGraphQuery {
    pub steps: Vec<RawStep>
}

/*
scope of the query.  determines if we're looking at the
entire graph or just from a handful of vertices
*/
pub enum Scope {
    Global,
    Vertex(Vec<i64>),
}

/*
generic step used in ParsedGraphQuery
will be turned into specific steps
*/
#[derive(Debug)]
pub struct RawStep {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, Display)]
pub enum Arg {
    Integer(i64),
    Float(f64),
    String(String),
}

impl RawStep {
    pub fn new(name: String, args: Vec<Arg>) -> RawStep {
        RawStep{name:name, args:args}
    }
}

impl fmt::Display for RawStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RawStep {}", self.name)
    }
}
