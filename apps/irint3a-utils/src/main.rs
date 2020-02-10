extern crate clap;

use clap::{App, Arg};
use std::io::Read;
use std::io::Write;

use interp_irint3a::runtime;
use irint3a::irparser::Parser;
use irint3a::irprinter::CodePrintable;

fn set_stdin(rt: &mut interp_irint3a::runtime::Runtime, path: &str) {
    if path == "-" {
        let mut data = vec![];
        std::io::stdin().read_to_end(&mut data).unwrap();
        rt.reset_stdin_raw(&data);
    } else {
        rt.reset_stdin_path(path);
    }
}

fn main() {
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
        .arg(
            Arg::with_name("stdin")
                .long("stdin")
                .value_name("FILE")
                .help("Set the stdin file for the interpreter environment")
                .takes_value(true),
        )
        .get_matches();

    let in_path = matches.value_of("INPUT").unwrap();
    let ps = Parser::from_file(&in_path);
    let (code, names) = ps.build();

    if matches.occurrences_of("dump") > 0 {
        code.print_code(&mut std::io::stdout(), Some(&names));
    }

    if matches.occurrences_of("run") > 0 {
        let mut rt = runtime::Runtime::new(code);

        if let Some(stdin_path) = matches.value_of("stdin") {
            set_stdin(&mut rt, stdin_path);
        }

        let ret_code = rt.run();
        std::io::stdout().write_all(rt.stdout()).unwrap();
        std::process::exit(ret_code.get_val());
    }
}
