
use crate::stream::Stream;
use crate::token::Token;
use crate::trie::Trie;
use std::collections::HashSet;

pub struct Lexer
{
    is: Stream,
    tok: Option<Token>,
    kws_set: HashSet<&'static str>,
    syms_trie: Trie,
}

impl Lexer {

    pub fn new(is: Stream, kws: Vec<&'static str>, syms: Vec<&str>) -> Lexer {
	Lexer {
	    is,
	    tok: None,
	    kws_set: kws.into_iter().collect(),
	    syms_trie: Trie::from_words(syms),
	}
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
		let res = self.parse_token();
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

    fn parse_token(&mut self) -> Token {

	loop {
	    if self.is.eof() {
		return Token::EOF
	    };

	    let c = self.is.get_char();
	    if !c.is_whitespace() {
		break;
	    }
	    self.is.next_char();
	}
	
	let c = self.is.get_char();
	if Lexer::char_is_id_start(c) {
	    self.parse_token_id()
	}
	else if Lexer::char_isnumber(c) {
	    self.parse_token_num()
	}
	else if c == '"' {
	    self.parse_token_str()
	}
	else if self.syms_trie.can_start_with(c) {
	    self.parse_token_sym()
	}
	else {
	    panic!("Invalid char: '{}'", c);
	}
    }

    fn parse_token_id(&mut self) -> Token {
	let mut name = String::new();
	loop {

	    let c = self.is.get_char();
	    if !Lexer::char_is_id(c) {
		break;
	    }
	    self.is.next_char();
	    name.push(c);
	}

	if name.len() == 0 {
	    panic!("Empty string");
	}

	if self.kws_set.contains(&name[..]) {
	    return Token::Keyword(name);
	}

	Token::Id(name)
    }

    fn parse_token_num(&mut self) -> Token {

	let mut val = String::new();
	let mut dec = false;

	loop {

	    let c = self.is.get_char();
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

	    self.is.next_char();
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

    fn parse_token_str(&mut self) -> Token {
	if self.is.get_char() != '"' {
	    panic!("Exptected '\"'");
	}
	self.is.next_char();

	let mut str = String::new();

	loop {
	    if self.is.eof() {
		panic!("End of file in string");
	    }
	    let c = self.is.get_char();
	    self.is.next_char();

	    if c == '\n' {
		panic!("Newline in string");
	    }
	    if c == '"' {
		break;
	    }

	    str.push(c);
	}
	
	
	Token::ValString(str)
    }

    fn parse_token_sym(&mut self) -> Token {
	let c = self.is.get_char();
	let mut val = String::new();
	val.push(c);
	let res = Token::Symbol(val);
	self.is.next_char();
	res
    }
}
