#![plugin(peg_syntax_ext)]

use steps::GraphQuery;

peg_file! gremlin("gremlin.rustpeg");

pub fn parse(g: &str) -> Result<GraphQuery, gremlin::ParseError> {
    gremlin::query(g)
}
