use oblexer::token::Token;

pub struct Parser {
    ps: obparser::parser::Parser,
}

impl Parser {
    fn kws() -> Vec<&'static str> {
        vec![
            "define", "extern", "movi", "movr", "load", "store", "alloca", "add", "sub", "mul",
            "div", "mod", "cmpeq", "cmplt", "cmpgt", "jump", "br", "call", "ret",
        ]
    }

    fn syms() -> Vec<&'static str> {
        vec!["%", ",", ":"]
    }

    pub fn new_from_file(path: &str) -> Parser {
        Parser {
            ps: obparser::parser::Parser::new_from_file(path, Parser::kws(), Parser::syms()),
        }
    }

    pub fn new_from_str(path: &str) -> Parser {
        Parser {
            ps: obparser::parser::Parser::new_from_str(path, Parser::kws(), Parser::syms()),
        }
    }

    /*
    pub fn parse(&mut self) -> ast::ASTExprPtr {
        self.r_file()
    }
    */
}
