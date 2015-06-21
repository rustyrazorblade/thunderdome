

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
    #[test]
    fn vertex_query() {
        let r = parse("g.v(1)");
        assert!(r.is_ok());
        let r = parse("g.v(1,2)");
        assert!(r.is_ok());
        assert!(parse("g.v(1, 2)").is_ok());
    }

}
