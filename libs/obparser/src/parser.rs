
use oblexer::lexer::Lexer;
use oblexer::stream::Stream;
use oblexer::token::Token;

pub struct Parser
{
    lex: Lexer
}

impl Parser {

    pub fn new_from_file(path: &str, kws: Vec<&'static str>, syms: Vec<&'static str>) -> Parser {

	let is = Stream::new(path);
	let lex = Lexer::new(is, kws, syms);
	Parser {
	    lex
	}
    }

    pub fn peek_token(&mut self) -> Token {
	self.lex.peek()
    }

    pub fn get_token(&mut self) -> Token {
	self.lex.get()
    }

    pub fn eat_eof(&mut self)
    {
	let tok = self.get_token();
	if !tok.is_eof() {
	    panic!("Expected End Of File token, got '{:?}'", tok);
	}
    }

    pub fn eat_id(&mut self) -> String
    {
	let tok = self.get_token();
	match tok {
	    Token::Id(x) => x,
	    _ => panic!("Expected Id, got '{:?}'", tok)
	}
    }

    pub fn eat_keyword(&mut self, val: &str)
    {
	let tok = self.get_token();
	match tok {
	    Token::Keyword(x) if x == val => {},
	    _ => panic!("Expected keyword '{}', got '{:?}'", val, tok)
	}
    }

    pub fn eat_sym(&mut self, val: &str)
    {
	let tok = self.get_token();
	match tok {
	    Token::Symbol(x) if x == val => {},
	    _ => panic!("Expected symbol '{}', got '{:?}'", val, tok)
	}
    }

    pub fn eat_vint(&mut self) -> u64
    {
	let tok = self.get_token();
	match tok {
	    Token::ValInt(x) => x,
	    _ => panic!("Expected int value, got '{:?}'", tok)
	}
    }

    pub fn eat_vfloat(&mut self) -> f64
    {
	let tok = self.get_token();
	match tok {
	    Token::ValFloat(x) => x,
	    _ => panic!("Expected float value, got '{:?}'", tok)
	}
    }

    pub fn eat_vstring(&mut self) -> String
    {
	let tok = self.get_token();
	match tok {
	    Token::ValString(x) => x,
	    _ => panic!("Expected string value, got '{:?}'", tok)
	}
    }

    pub fn try_eat_eof(&mut self) -> bool {
	if self.peek_token().is_eof() {
		self.lex.get();
		true
	}
	else {
	    false 
	}
    }

    pub fn try_eat_id(&mut self) -> Option<String> {
	match self.peek_token() {
	    Token::Id(x) => {
		self.lex.get();
		Some(x)
	    },
	    _ => None
	}
    }

    pub fn try_eat_keyword(&mut self, val: &str) -> bool {
	match self.peek_token() {
	    Token::Keyword(x) if x == val => {
		self.lex.get();
		true
	    },
	    _ => false 
	}
    }

    pub fn try_eat_sym(&mut self, val: &str) -> bool {
	match self.peek_token() {
	    Token::Symbol(x) if x == val => {
		self.lex.get();
		true
	    },
	    _ => false 
	}
    }

    pub fn try_eat_vint(&mut self) -> Option<u64> {
	match self.peek_token() {
	    Token::ValInt(x) => {
		self.lex.get();
		Some(x)
	    },
	    _ => None
	}
    }

    pub fn try_eat_vfloat(&mut self) -> Option<f64> {
	match self.peek_token() {
	    Token::ValFloat(x) => {
		self.lex.get();
		Some(x)
	    },
	    _ => None
	}
    }

    pub fn try_eat_vstring(&mut self) -> Option<String> {
	match self.peek_token() {
	    Token::ValString(x) => {
		self.lex.get();
		Some(x)
	    },
	    _ => None
	}
    }
    

}
