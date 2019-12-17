
#[derive(Clone)]
#[derive(Debug)]
pub enum Token
{
    EOF,
    Id(String),
    Keyword(String),
    Symbol(String),
    ValInt(u64),
    ValFloat(f64),
    ValString(String),
}

impl Token {


    pub fn is_eof(&self) -> bool {
	match self {
	    Token::EOF => true,
	    _ => false
	}
    }
    
}
