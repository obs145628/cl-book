use irint3a::irparser::Parser;
use irint3a::irprinter::CodePrintable;

use std::env;

fn main() {
    let in_path = env::args()
        .nth(1)
        .expect("Usage: ./irint3a-utils <inout-file>");

    let ps = Parser::new_from_file(&in_path);
    let code = ps.parse();
    code.print_code(&mut std::io::stdout());
}
