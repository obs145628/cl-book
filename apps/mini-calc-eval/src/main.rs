mod ast;
mod parser;
mod value;

use std::env;

fn main() {
    match env::args().nth(1) {
        None => {
            println!("Usage: {} <expr>", env::args().next().unwrap());
        }

        Some(expr) => {
            let ast = parser::ParserAST::new_from_str(&expr).build();
            ast.dump();
            println!("");

            let mut rt = parser::ParserEval::new_from_str(&expr);
            match rt.eval() {
                value::Value::VInt(x) => println!("{}", x),
                value::Value::VFloat(x) => println!("{}", x),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_file(path: &str) -> value::Value {
        parser::ParserEval::new_from_file(path).eval()
    }

    fn eval_file_from_str(path: &str) -> value::Value {
        let data = std::fs::read_to_string(path).unwrap();
        parser::ParserEval::new_from_str(&data).eval()
    }

    #[test]
    fn test_c1() {
        assert_eq!(eval_file("./ex/c1.txt").get_int(), 34);
        assert_eq!(eval_file_from_str("./ex/c1.txt").get_int(), 34);
    }

    #[test]
    fn test_c2() {
        assert_eq!(eval_file("./ex/c2.txt").get_float(), 18.5);
        assert_eq!(eval_file_from_str("./ex/c2.txt").get_float(), 18.5);
    }

    #[test]
    fn test_addsub1() {
        assert_eq!(eval_file("./ex/addsub1.txt").get_int(), 15);
        assert_eq!(eval_file_from_str("./ex/addsub1.txt").get_int(), 15);
    }

    #[test]
    fn test_muldiv1() {
        assert_eq!(eval_file("./ex/muldiv1.txt").get_int(), 2);
        assert_eq!(eval_file_from_str("./ex/muldiv1.txt").get_int(), 2);
    }

    #[test]
    fn test_unary1() {
        assert_eq!(eval_file("./ex/unary1.txt").get_int(), -12);
        assert_eq!(eval_file_from_str("./ex/unary1.txt").get_int(), -12);
    }

    #[test]
    fn test_paren1() {
        assert_eq!(eval_file("./ex/paren1.txt").get_int(), 29);
        assert_eq!(eval_file_from_str("./ex/unary1.txt").get_int(), -12);
    }

    #[test]
    fn test_ops1() {
        assert_eq!(eval_file("./ex/ops1.txt").get_int(), -41);
        assert_eq!(eval_file_from_str("./ex/ops1.txt").get_int(), -41);
    }

    #[test]
    fn test_ops2() {
        assert_eq!(eval_file("./ex/ops2.txt").get_float(), 3.5);
        assert_eq!(eval_file_from_str("./ex/ops2.txt").get_float(), 3.5);
    }
}
