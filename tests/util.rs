extern crate thunderdome;

use thunderdome::graph::*;
use thunderdome::path::Element;

fn simple_graph() -> Box<Graph> {
    let mut g = Graph::new();
    let mut jon = g.add_vertex();
    let mut blake = g.add_vertex();
    let mut caleb = g.add_vertex();
    let mut mike = g.add_vertex();

    jon.add_edge(&mut blake, "friend");
    jon.add_edge(&mut caleb, "friend");
    blake.add_edge(&mut caleb, "friend");
    blake.add_edge(&mut jon, "tried_to_murder");

    jon.add_edge(&mut mike, "enemy");
    g
}
