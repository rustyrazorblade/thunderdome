extern crate thunderdome;


#[cfg(test)]
mod parser_tests {
    use thunderdome::parser::*;

    fn validate(q: &str) -> Option<ParsedGraphQuery> {
        let result = pre_parse(q);
        assert!(result.is_ok());
        result.ok()
    }

    #[test]
    fn global_graph_query() {
        validate("g.V()");
    }

    #[test]
    fn vertex_query() {
        validate("g.v(1)");
        validate("g.v(1,2)");
        validate("g.v(1, 2)");
    }

    #[test]
    fn simple_step_test() {
        validate("g.v(1).outV()");
        let result = validate("g.v(1).outV().inE()").unwrap();
        assert_eq!(result.steps.len(), 3);

        let step1 = result.steps.get(1).unwrap();
        assert_eq!(step1.name, "outV".to_string());

        let step2 = result.steps.get(2).unwrap();
        assert_eq!(step2.name, "inE".to_string());
    }

    #[test]
    fn test_args() {
        let result = validate("g.v(1).outV('edge').has('age', 30)").unwrap();
        let step1 = result.steps.get(1).unwrap();
        assert_eq!(step1.name, "outV".to_string());
        // make sure the arg is edge.  should be a string and unquoted
        match step1.args.get(0).unwrap() {
            &Arg::String(ref x) if *x == "edge".to_string() => {},
            &Arg::String(ref x) => panic!("{}", x),
            x => { panic!("wrong type") }
        }
    }
    #[test]
    fn test_args_numbers() {
        // maybe a weird offset command?
        validate("g.V().limit(10, 20)");
        let result = validate("g.V().limit(10.0)").unwrap();
        match result.steps.get(1).unwrap().args.get(0).unwrap() {
            &Arg::Float(ref x) if *x == 10.0 => { },
            _ => { panic!("OH NOES")}
        }
    }
}
