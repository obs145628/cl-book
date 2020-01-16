mod ast;
mod astcast;
mod defstable;
mod letype;
mod parser;
mod typecheck;

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    fn check_parser(path: &str) {
        let mut ps = parser::Parser::new_from_file(path);
        let _ast = ps.parse();
    }

    fn check_type(path: &str) {
        let mut ps = parser::Parser::new_from_file(path);
        let ast = ps.parse();

        let mut tc = typecheck::TypeCheck::new();
        tc.check(&ast);
    }

    fn list_files(dir: &str) -> Vec<String> {
        fs::read_dir(dir)
            .unwrap()
            .map(|x| x.unwrap().path())
            .filter(|x| x.is_file())
            .map(|x| x.to_str().unwrap().to_string())
            .filter(|x| x.ends_with(".le"))
            .collect()
    }

    #[test]
    fn test_parser_grammar() {
        let files = list_files("./tests/grammar/");
        for f in files {
            println!("running {}...", f);
            check_parser(&f);
        }
    }

    #[test]
    fn test_parser_basics() {
        let files = list_files("./tests/basics/");
        for f in files {
            println!("running {}...", f);
            check_parser(&f);
        }
    }

    #[test]
    fn test_type_basics() {
        let files = list_files("./tests/basics/");
        for f in files {
            println!("running {}...", f);
            check_type(&f);
        }
    }
}
