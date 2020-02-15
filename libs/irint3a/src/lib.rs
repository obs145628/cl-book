pub mod ir;
pub mod irbuilder;
pub mod irnames;
pub mod irparser;
pub mod irprinter;
pub mod irvalidation;
pub mod registers;

pub mod controlflow;
pub mod digraph;

#[cfg(test)]
mod tests {

    use super::*;

    fn test_lexer_printer(path: &str) {
        use crate::irprinter::CodePrintable;

        let ps = irparser::Parser::from_file(path);
        let (code, names) = ps.build();

        let mut code_str: Vec<u8> = vec![];
        code.print_code(&mut code_str, Some(&names));
        let code_str = std::str::from_utf8(&code_str).unwrap();
        println!("CODE1: <BEG>{}<END>", code_str);

        let ps2 = irparser::Parser::from_str(code_str);
        let (code, names) = ps2.build();

        let mut code2_str: Vec<u8> = vec![];
        code.print_code(&mut code2_str, Some(&names));
        let code2_str = std::str::from_utf8(&code2_str).unwrap();
        println!("CODE2: <BEG>{}<END>", code2_str);

        assert_eq!(code_str, code2_str);
    }

    #[test]
    fn lexer_printer_fn_add() {
        test_lexer_printer("./tests/fn_add.ir");
    }

    #[test]
    fn lexer_printer_fn_fact() {
        test_lexer_printer("./tests/fn_fact.ir");
    }

    #[test]
    fn lexer_printer_hello_42() {
        test_lexer_printer("./tests/hello_42.ir");
    }
}
