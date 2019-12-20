use oblexer::lexer::Lexer;
use oblexer::stream::Stream;
use oblexer::token::Token;
use crate::value::Value;

pub struct Parser
{
    lex: Lexer
}

impl Parser
{

    pub fn new(path: &str) -> Parser {

	let kws = vec![];
	let syms = vec!["+", "-", "*", "/", "(", ")"];
	
	Parser {
	    lex: Lexer::new(Stream::new(path), kws, syms),
	}
    }

    pub fn eat_eof(&mut self)
    {
	let tok = self.lex.get();
	if !tok.is_eof() {
	    panic!("Expected End Of File token, got '{:?}'", tok);
	}
    }

    pub fn eat_sym(&mut self, val: &str)
    {
	let tok = self.lex.get();
	match tok {
	    Token::Symbol(x) if x == val => {},
	    _ => panic!("Expected symbol '{}', got '{:?}'", val, tok)
	}
    }

    pub fn try_eat_sym(&mut self, val: &str) -> bool {
	match self.lex.peek() {
	    Token::Symbol(x) if x == val => {
		self.lex.get();
		true
	    },
	    _ => false 
	}
    }

    pub fn eval(&mut self) -> Value {
	let res = self.eval_expr();
	self.eat_eof();
	res
    }

    pub fn eval_expr(&mut self) -> Value {
	self.eval_l2()
    }

    // + -
    pub fn eval_l2(&mut self) -> Value {
	let mut res = self.eval_l1();

	loop {
	    match self.lex.peek() {
		Token::Symbol(x) if x == "+" => {
		    self.lex.get();
		    res = Value::add(&res, &self.eval_l1());
		}

		Token::Symbol(x) if x == "-" => {
		    self.lex.get();
		    res = Value::sub(&res, &self.eval_l1());
		}

		_ => { break; }
	    }
	}

	res
    }

    // * /
    pub fn eval_l1(&mut self) -> Value {
	let mut res = self.eval_l0();

	loop {
	    match self.lex.peek() {
		Token::Symbol(x) if x == "*" => {
		    self.lex.get();
		    res = Value::mul(&res, &self.eval_l0());
		}

		Token::Symbol(x) if x == "/" => {
		    self.lex.get();
		    res = Value::div(&res, &self.eval_l0());
		}

		_ => { break; }
	    }
	}

	res
    }

    // unary + -
    pub fn eval_l0(&mut self) -> Value {

	match self.lex.peek() {
	    Token::Symbol(x) if x == "+" => {
		self.lex.get();
		self.eval_l0()
	    }

	    Token::Symbol(x) if x == "-" => {
		self.lex.get();
		Value::sub(&Value::VInt(0), &self.eval_l1())
	    },

	    _ => self.eval_prim()
	}
    }

    // ( <expr> ) or const
    pub fn eval_prim(&mut self) -> Value {

	if !self.try_eat_sym("(") {
	    return self.eval_const();
	}
	
	let res = self.eval_expr();
	self.eat_sym(")");
	res
    }
    

    fn eval_const(&mut self) -> Value {
	match self.lex.get() {
	    Token::ValInt(x) => Value::VInt(x as i64),
	    Token::ValFloat(x) => Value::VFloat(x),
	    _ => panic!("Expected int or float token")
	}
    }
    

}
