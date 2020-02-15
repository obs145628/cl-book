extern crate clap;

use clap::{App, Arg};
use std::collections::HashMap;
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
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Set the program output file")
                .takes_value(true),
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
        .arg(
            Arg::with_name("dump-cfg")
                .long("dump-cfg")
                .value_name("FUNCTION")
                .help("Create a dot output file for the CFG of the corresponding function")
                .takes_value(true),
        )
        .get_matches();

    let in_path = matches.value_of("INPUT").unwrap();
    let out_path = matches.value_of("OUTPUT");
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

    if let Some(cfg_fname) = matches.value_of("dump-cfg") {
        let out_path = out_path.unwrap_or("cfg.dot");
        let fun_id = names
            .get_function_id(&cfg_fname)
            .expect("dump-cfg: function not found");
        let fun = code.get_fun(fun_id).unwrap();
        let fun_names = names.get_function(fun_id).unwrap();

        let cfg = irint3a::controlflow::build_cfg(fun);

        let fun_names: HashMap<usize, String> = fun
            .basic_blocks_list()
            .iter()
            .map(|bb_id| {
                (
                    bb_id.0,
                    fun_names.get_basic_block_name(*bb_id).unwrap().to_string(),
                )
            })
            .collect();

        cfg.write_dot(out_path, Some("cfg"), Some(&fun_names));
    }
}
