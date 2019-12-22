mod parser;
mod value;

use std::env;

fn main() {
    match env::args().nth(1) {
        None => {
            println!("Usage: {} <expr>", env::args().next().unwrap());
        }

        Some(expr) => {
            let mut rt = parser::Parser::new_from_str(&expr);
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
        parser::Parser::new_from_file(path).eval()
    }

    #[test]
    fn test_c1() {
        assert_eq!(eval_file("./ex/c1.txt").get_int(), 34);
    }

    #[test]
    fn test_c2() {
        assert_eq!(eval_file("./ex/c2.txt").get_float(), 18.5);
    }

    #[test]
    fn test_addsub1() {
        assert_eq!(eval_file("./ex/addsub1.txt").get_int(), 15);
    }

    #[test]
    fn test_muldiv1() {
        assert_eq!(eval_file("./ex/muldiv1.txt").get_int(), 2);
    }

    #[test]
    fn test_unary1() {
        assert_eq!(eval_file("./ex/unary1.txt").get_int(), -12);
    }

    #[test]
    fn test_paren1() {
        assert_eq!(eval_file("./ex/paren1.txt").get_int(), 29);
    }

    #[test]
    fn test_ops1() {
        assert_eq!(eval_file("./ex/ops1.txt").get_int(), -41);
    }

    #[test]
    fn test_ops2() {
        assert_eq!(eval_file("./ex/ops2.txt").get_float(), 3.5);
    }
}
