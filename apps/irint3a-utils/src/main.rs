extern crate clap;

use clap::{App, Arg};
use std::io::Write;

use interp_irint3a::runtime;
//use irint3a::irparser::Parser;
use irint3a::irprinter::CodePrintable;

fn main() {
    /*
    let matches = App::new("irint3a-utils")
        .version("0.1.0")
        .author("Steven Lariau <obs145628@gmail.com>")
        .about("Utils to manipulate irint3a IR files")
        .arg(
            Arg::with_name("INPUT")
                .help("Set the input file")
                .required(true),
        )
        .arg(Arg::with_name("dump").long("dump").help("Dump the IR"))
        .arg(
            Arg::with_name("run")
                .long("run")
                .help("Run the IR program with an interpreter"),
        )
        .get_matches();

    let in_path = matches.value_of("INPUT").unwrap();
    let ps = Parser::new_from_file(&in_path);
    let code = ps.parse();

    if matches.occurrences_of("dump") > 0 {
        code.print_code(&mut std::io::stdout());
        println!("\n");
    }

    if matches.occurrences_of("run") > 0 {
        let code = code.keep_module();
        let mut rt = runtime::Runtime::new(code);
        let ret_code = rt.run();
        std::io::stdout().write_all(rt.stdout()).unwrap();
        std::process::exit(ret_code.get_val());
    }
    */
}
