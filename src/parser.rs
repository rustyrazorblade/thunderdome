#![plugin(peg_syntax_ext)]

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

struct GraphQueryResult;

impl ParsedGraphQuery {
    fn execute(&self) -> Result<GraphQueryResult, &str> {
        // get the starting point(s)

        // i'm tempted to say global graph traversals must include a
        // starting point, g.v(predicate)

        // execute each of the steps sequentially
        for step in self.steps.iter() {
            // lookup step in hash table
            // execute function, passing step args
        }
        Result::Err("meh")
    }
}
