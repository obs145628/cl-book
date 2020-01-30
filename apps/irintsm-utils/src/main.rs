use std::env;

use irintsm::irparser::Parser;
use irintsm::irprinter::CodePrintable;

fn main() {
    let in_path = env::args()
        .nth(1)
        .expect("Usage: ./irintsm-utils <input-file>");

    let ps = Parser::from_file(&in_path);
    let code = ps.build();
    code.print_code(&mut std::io::stdout());
}
