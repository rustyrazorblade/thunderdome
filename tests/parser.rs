

extern crate thunderdome;

#[cfg(test)]
mod parser_tests {
    use thunderdome::parser::*;

    #[test]
    fn global_graph_query() {
        parse("g");
        let result = parse("g.V");
        assert!(result.is_ok());

    }

}
