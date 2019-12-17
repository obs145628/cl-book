
use crate::stream::Stream;
use crate::token::Token;

pub struct Lexer
{
    is: Stream,
    tok: Option<Token>,
}

impl Lexer {

    pub fn new(is: Stream) -> Lexer {
	Lexer {
	    is,
	    tok: None,
	}
    }

    pub fn new_from_file(path: &str) -> Lexer {
	Lexer::new(Stream::new(path))
    }
    

    pub fn peek(&mut self) -> Token {
	self.get_last()
    }

    pub fn get(&mut self) -> Token {
	let res = self.get_last();
	self.invalid_last();
	res
    }

    fn get_last(&mut self) -> Token {

	match &self.tok {
	    None => {
		let res = Lexer::parse_token(&mut self.is);
		self.tok = Some(res.clone());
		res
	    },
	    Some(tok) => tok.clone()
	}
    }

    fn invalid_last(&mut self) {
	if let Some(tok) = &self.tok {
	    if !tok.is_eof() {
		self.tok = None
	    }
	};
    }

    fn char_is_id(c: char) -> bool {
	c.is_alphanumeric() || c == '_'
    }

    fn char_is_id_start(c: char) -> bool {
	Lexer::char_is_id(c) && !c.is_numeric()
    }

    fn char_isnumber(c: char) -> bool {
	c.is_numeric() || c == '.'
    }

    fn parse_token(is: &mut Stream) -> Token {

	loop {
	    if is.eof() {
		return Token::EOF
	    };

	    let c = is.get_char();
	    if !c.is_whitespace() {
		break;
	    }
	    is.next_char();
	}
	
	let c = is.get_char();
	if Lexer::char_is_id_start(c) {
	    Lexer::parse_token_id(is)
	}
	else if Lexer::char_isnumber(c) {
	    Lexer::parse_token_num(is)
	}
	else if c == '"' {
	    Lexer::parse_token_str(is)
	}
	else {
	    Lexer::parse_token_sym(is)
	}
    }

    fn parse_token_id(is: &mut Stream) -> Token {
	Token::EOF
    }

    fn parse_token_num(is: &mut Stream) -> Token {

	let mut val = String::new();
	let mut dec = false;

	loop {

	    let c = is.get_char();
	    if c.is_numeric() {
		
	    }
	    else if c == '.' {
		if dec {
		    panic!("Second '.' in number");
		}
		dec = true;
	    }
	    else {
		break;
	    }

	    is.next_char();
	    val.push(c);
	}

	if val.len() == 0 {
	    panic!("Received empty string")
	}

	if dec {
	    let x : f64 = val.parse().unwrap();
	    Token::ValFloat(x)
	}
	else {
	    let x : u64 = val.parse().unwrap();
	    Token::ValInt(x)
	}
    }

    fn parse_token_str(is: &mut Stream) -> Token {
	Token::EOF
    }

    fn parse_token_sym(is: &mut Stream) -> Token {
	let c = is.get_char();
	let mut val = String::new();
	val.push(c);
	let res = Token::Symbol(val);
	is.next_char();
	res
    }
}
