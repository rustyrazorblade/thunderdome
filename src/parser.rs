#![plugin(peg_syntax_ext)]

use steps::ParsedGraphQuery;

peg_file! gremlin("gremlin.rustpeg");

pub fn parse(g: &str) -> Result<ParsedGraphQuery, gremlin::ParseError> {
    pre_parse(g)
}

pub fn pre_parse(g: &str) -> Result<ParsedGraphQuery, gremlin::ParseError> {
    gremlin::query(g)
}
