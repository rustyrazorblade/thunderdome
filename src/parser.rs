#![feature(plugin)]
#![plugin(peg_syntax_ext)]

use steps::GraphQuery;

use gremlin::*;
peg_file! gremlin("gremlin.rustpeg");

fn parse(g: &str) -> GraphQuery {
    query(g)
}
