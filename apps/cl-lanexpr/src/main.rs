extern crate clap;

use clap::{App, Arg};
use lanexpr::ast;
use lanexpr::bindapp::BindApp;
use lanexpr::parser;
use lanexpr::translater;
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

fn gen_irint3a(root: &ast::ASTExprPtr, ba: &BindApp) {
    use irint3a::irprinter::CodePrintable;

    let tr = translater::irint3a::Translater::new(root, ba);
    let code = tr.translate();
    code.print_code(&mut std::io::stdout());
}

fn gen_irintsm(root: &ast::ASTExprPtr, ba: &BindApp) {
    use irintsm::irprinter::CodePrintable;

    let tr = translater::irintsmtl::Translater::new(root, ba);
    let code = tr.translate();
    code.print_code(&mut std::io::stdout());
}

fn gen_llvm_ir(root: &ast::ASTExprPtr, ba: &BindApp, out_path: Option<&str>) {
    let mut tr = translater::llvmtl::Translater::new(root, ba);
    if let Some(out_path) = out_path {
        tr.set_output_ll_path(out_path);
    }
    tr.translate();
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
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Set the program output file")
                .takes_value(true),
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
        .arg(
            Arg::with_name("gen-irint3a")
                .long("gen-irint3a")
                .help("Generate IR code for irint3a"),
        )
        .arg(
            Arg::with_name("gen-irintsm")
                .long("gen-irintsm")
                .help("Generate IR code for irintsm"),
        )
        .arg(
            Arg::with_name("gen-llvm")
                .long("gen-llvm")
                .help("Generate LLVM IR code"),
        )
        .arg(
            Arg::with_name("bin-llvm")
                .long("bin-llvm")
                .help("Create an executable by generating LLVM IR"),
        )
        .get_matches();

    let input_path = matches.value_of("INPUT").unwrap();
    let output_path = matches.value_of("OUTPUT");

    if matches.occurrences_of("stage-parse") > 0 {
        do_parse(input_path);
    } else if matches.occurrences_of("stage-type") > 0 {
        let ast = do_parse(input_path);
        do_typecheck(&ast);
    } else if matches.occurrences_of("dump-bindings") > 0 {
        let ast = do_parse(input_path);
        let ati = do_typecheck(&ast);
        ati.dump_bindings();
    } else if matches.occurrences_of("gen-irint3a") > 0 {
        let ast = do_parse(input_path);
        let ati = do_typecheck(&ast);
        gen_irint3a(&ast, &ati);
    } else if matches.occurrences_of("gen-irintsm") > 0 {
        let ast = do_parse(input_path);
        let ati = do_typecheck(&ast);
        gen_irintsm(&ast, &ati);
    } else if matches.occurrences_of("gen-llvm") > 0 {
        let ast = do_parse(input_path);
        let ati = do_typecheck(&ast);
        gen_llvm_ir(&ast, &ati, output_path);
    } else if matches.occurrences_of("bin-llvm") > 0 {
        let output_path = output_path.unwrap_or("./a.out");
        let ast = do_parse(input_path);
        let ati = do_typecheck(&ast);
        translater::llvmbin::compile_to_binary(&ast, &ati, output_path);
    }
}
