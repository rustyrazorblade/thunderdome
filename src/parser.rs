#![plugin(peg_syntax_ext)]

use steps::ParsedGraphQuery;

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
