

extern crate thunderdome;

#[cfg(test)]
mod parser_tests {
    use thunderdome::parser::*;

    fn global_graph_query() {
        let result = parse("g.V");
    }

}
