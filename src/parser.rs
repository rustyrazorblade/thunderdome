#![feature(plugin)]
#![plugin(peg_syntax_ext)]

use steps::GraphQuery;

peg_file! gremlin("gremlin.rustpeg");

// found `core::result::Result<steps::GraphQuery, parser::gremlin::ParseError>`
pub fn parse(g: &str) -> Result<GraphQuery, gremlin::ParseError> {
    gremlin::query(g)
}
