
mod parser;
mod value;


fn main() {
    let mut rt = parser::Parser::new("./ex/paren1.txt");
    println!("{:?}", rt.eval());
}




#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_c1() {
	let mut rt = parser::Parser::new("./ex/c1.txt");
        assert_eq!(rt.eval().get_int(), 34);
    }

    #[test]
    fn test_c2() {
	let mut rt = parser::Parser::new("./ex/c2.txt");
        assert_eq!(rt.eval().get_float(),  18.5);
    }

    #[test]
    fn test_addsub1() {
	let mut rt = parser::Parser::new("./ex/addsub1.txt");
        assert_eq!(rt.eval().get_int(),  15);
    }

    #[test]
    fn test_muldiv1() {
	let mut rt = parser::Parser::new("./ex/muldiv1.txt");
        assert_eq!(rt.eval().get_int(),  2);
    }

    #[test]
    fn test_unary1() {
	let mut rt = parser::Parser::new("./ex/unary1.txt");
        assert_eq!(rt.eval().get_int(),  -12);
    }

    #[test]
    fn test_paren1() {
	let mut rt = parser::Parser::new("./ex/paren1.txt");
        assert_eq!(rt.eval().get_int(),  29);
    }
}
