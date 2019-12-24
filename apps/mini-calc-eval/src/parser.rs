use crate::ast::{ASTBinOp, ASTBinopOp, ASTConst, ASTPtr, ASTUnop};
use crate::value::Value;
use oblexer::token::Token;

pub struct ParserEval {
    ps: obparser::parser::Parser,
}

impl ParserEval {
    fn kws() -> Vec<&'static str> {
        vec![]
    }

    fn syms() -> Vec<&'static str> {
        vec!["+", "-", "*", "/", "(", ")"]
    }

    pub fn new_from_file(path: &str) -> ParserEval {
        ParserEval {
            ps: obparser::parser::Parser::new_from_file(
                path,
                ParserEval::kws(),
                ParserEval::syms(),
            ),
        }
    }

    pub fn new_from_str(path: &str) -> ParserEval {
        ParserEval {
            ps: obparser::parser::Parser::new_from_str(path, ParserEval::kws(), ParserEval::syms()),
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
                Value::sub(&Value::VInt(0), &self.eval_l0())
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

pub struct ParserAST {
    ps: obparser::parser::Parser,
}

impl ParserAST {
    fn kws() -> Vec<&'static str> {
        vec![]
    }

    fn syms() -> Vec<&'static str> {
        vec!["+", "-", "*", "/", "(", ")"]
    }

    pub fn new_from_file(path: &str) -> ParserAST {
        ParserAST {
            ps: obparser::parser::Parser::new_from_file(path, ParserAST::kws(), ParserAST::syms()),
        }
    }

    pub fn new_from_str(path: &str) -> ParserAST {
        ParserAST {
            ps: obparser::parser::Parser::new_from_str(path, ParserAST::kws(), ParserAST::syms()),
        }
    }

    pub fn build(&mut self) -> ASTPtr {
        let res = self.r_expr();
        self.ps.eat_eof();
        res
    }

    fn r_expr(&mut self) -> ASTPtr {
        self.r_l2()
    }

    // + -
    fn r_l2(&mut self) -> ASTPtr {
        let mut res = self.r_l1();

        loop {
            match self.ps.peek_token() {
                Token::Symbol(x) if x == "+" => {
                    self.ps.get_token();
                    res = ASTBinOp::new(ASTBinopOp::Add, res, self.r_l1());
                }

                Token::Symbol(x) if x == "-" => {
                    self.ps.get_token();
                    res = ASTBinOp::new(ASTBinopOp::Sub, res, self.r_l1());
                }

                _ => {
                    break;
                }
            }
        }

        res
    }

    // * /
    fn r_l1(&mut self) -> ASTPtr {
        let mut res = self.r_l0();

        loop {
            match self.ps.peek_token() {
                Token::Symbol(x) if x == "*" => {
                    self.ps.get_token();
                    res = ASTBinOp::new(ASTBinopOp::Mul, res, self.r_l0());
                }

                Token::Symbol(x) if x == "/" => {
                    self.ps.get_token();
                    res = ASTBinOp::new(ASTBinopOp::Div, res, self.r_l0());
                }

                _ => {
                    break;
                }
            }
        }

        res
    }

    // unary + -
    fn r_l0(&mut self) -> ASTPtr {
        match self.ps.peek_token() {
            Token::Symbol(x) if x == "+" => {
                self.ps.get_token();
                ASTUnop::new('+', self.r_l0())
            }

            Token::Symbol(x) if x == "-" => {
                self.ps.get_token();
                ASTUnop::new('-', self.r_l0())
            }

            _ => self.r_prim(),
        }
    }

    // ( <expr> ) or const
    fn r_prim(&mut self) -> ASTPtr {
        if !self.ps.try_eat_sym("(") {
            return self.r_const();
        }

        let res = self.r_expr();
        self.ps.eat_sym(")");
        res
    }

    fn r_const(&mut self) -> ASTPtr {
        match self.ps.get_token() {
            Token::ValInt(x) => ASTConst::new(Value::VInt(x as i64)),
            Token::ValFloat(x) => ASTConst::new(Value::VFloat(x)),
            _ => panic!("Expected int or float token"),
        }
    }
}
