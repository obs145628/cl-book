use crate::value::Value;
use oblexer::token::Token;

pub struct Parser {
    ps: obparser::parser::Parser,
}

impl Parser {
    fn kws() -> Vec<&'static str> {
        vec![]
    }

    fn syms() -> Vec<&'static str> {
        vec!["+", "-", "*", "/", "(", ")"]
    }

    pub fn new_from_file(path: &str) -> Parser {
        Parser {
            ps: obparser::parser::Parser::new_from_file(path, Parser::kws(), Parser::syms()),
        }
    }

    pub fn new_from_str(path: &str) -> Parser {
        Parser {
            ps: obparser::parser::Parser::new_from_str(path, Parser::kws(), Parser::syms()),
        }
    }

    pub fn eval(&mut self) -> Value {
        let res = self.eval_expr();
        self.ps.eat_eof();
        res
    }

    pub fn eval_expr(&mut self) -> Value {
        self.eval_l2()
    }

    // + -
    pub fn eval_l2(&mut self) -> Value {
        let mut res = self.eval_l1();

        loop {
            match self.ps.peek_token() {
                Token::Symbol(x) if x == "+" => {
                    self.ps.get_token();
                    res = Value::add(&res, &self.eval_l1());
                }

                Token::Symbol(x) if x == "-" => {
                    self.ps.get_token();
                    res = Value::sub(&res, &self.eval_l1());
                }

                _ => {
                    break;
                }
            }
        }

        res
    }

    // * /
    pub fn eval_l1(&mut self) -> Value {
        let mut res = self.eval_l0();

        loop {
            match self.ps.peek_token() {
                Token::Symbol(x) if x == "*" => {
                    self.ps.get_token();
                    res = Value::mul(&res, &self.eval_l0());
                }

                Token::Symbol(x) if x == "/" => {
                    self.ps.get_token();
                    res = Value::div(&res, &self.eval_l0());
                }

                _ => {
                    break;
                }
            }
        }

        res
    }

    // unary + -
    pub fn eval_l0(&mut self) -> Value {
        match self.ps.peek_token() {
            Token::Symbol(x) if x == "+" => {
                self.ps.get_token();
                self.eval_l0()
            }

            Token::Symbol(x) if x == "-" => {
                self.ps.get_token();
                Value::sub(&Value::VInt(0), &self.eval_l1())
            }

            _ => self.eval_prim(),
        }
    }

    // ( <expr> ) or const
    pub fn eval_prim(&mut self) -> Value {
        if !self.ps.try_eat_sym("(") {
            return self.eval_const();
        }

        let res = self.eval_expr();
        self.ps.eat_sym(")");
        res
    }

    fn eval_const(&mut self) -> Value {
        match self.ps.get_token() {
            Token::ValInt(x) => Value::VInt(x as i64),
            Token::ValFloat(x) => Value::VFloat(x),
            _ => panic!("Expected int or float token"),
        }
    }
}
