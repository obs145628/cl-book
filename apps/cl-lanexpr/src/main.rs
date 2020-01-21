extern crate clap;

use clap::{App, Arg};
use lanexpr::ast;
use lanexpr::bindapp::BindApp;
use lanexpr::parser;
use lanexpr::typecheck;

fn do_parse(path: &str) -> ast::ASTExprPtr {
    let mut ps = parser::Parser::new_from_file(path);
    ps.parse()
}

fn do_typecheck(ast: &ast::ASTExprPtr) -> BindApp {
    let mut tc = typecheck::TypeCheck::new();
    tc.check(&ast);
    tc.get_bindings()
}

fn main() {
    let matches = App::new("cl-lanexpr")
        .version("0.1.0")
        .author("Steven Lariau <obs145628@gmail.com>")
        .about("Compiler for lanexpr")
        .arg(
            Arg::with_name("INPUT")
                .help("Set the input file")
                .required(true),
        )
        .arg(
            Arg::with_name("stage-parse")
                .long("stage-parse")
                .help("Only parse the file"),
        )
        .arg(
            Arg::with_name("stage-type")
                .long("stage-type")
                .help("Only perform type-check"),
        )
        .arg(
            Arg::with_name("dump-bindings")
                .long("dump-bindings")
                .help("Only compoute and display bindings informations"),
        )
        .get_matches();

    let input_path = matches.value_of("INPUT").unwrap();

    if matches.occurrences_of("stage-parse") > 0 {
        do_parse(input_path);
    } else if matches.occurrences_of("stage-type") > 0 {
        let ast = do_parse(input_path);
        do_typecheck(&ast);
    } else if matches.occurrences_of("dump-bindings") > 0 {
        let ast = do_parse(input_path);
        let ati = do_typecheck(&ast);
        ati.dump_bindings();
    }
}
