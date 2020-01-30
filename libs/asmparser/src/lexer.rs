use oblexer::stream::Stream;

// Internal Lexer, start splitting input to make parser implem easier
// Detect:
// - label: labelname followed by :
// - comment: ; followed by anything till end of line
// - instruction: line of instruction
// - definiton: line starting with .

#[derive(Clone, Debug)]
pub enum Token {
    Label(String),
    Comment(String),
    Ins(String),
    Def(String),
    EOF,
}

pub struct Lexer {
    is: Stream,
    next: Option<Token>,
}

impl Lexer {
    pub fn from_file(path: &str) -> Self {
        Lexer {
            is: Stream::from_file(path),
            next: None,
        }
    }

    pub fn from_str(path: &str) -> Self {
        Lexer {
            is: Stream::from_str(path),
            next: None,
        }
    }

    pub fn peek(&mut self) -> &Token {
        self.load_next();
        self.next.as_ref().unwrap()
    }

    pub fn next(&mut self) -> Token {
        self.load_next();
        let mut res = None;
        std::mem::swap(&mut res, &mut self.next);
        res.unwrap()
    }

    fn load_next(&mut self) {
        if self.next.is_none() {
            self.next = Some(self.read_next());
        }
    }

    fn read_next(&mut self) -> Token {
        // 1) Skip whitespaces
        loop {
            match self.is.get_char() {
                None => {
                    return Token::EOF;
                }
                Some(c) if !c.is_whitespace() => {
                    break;
                }
                _ => {}
            }
            self.is.next_char();
        }

        // 2) Read comment
        if self.is.get_char().unwrap() == ';' {
            self.is.next_char();
            return Token::Comment(self.read_until("\n"));
        }

        // 3) Read definition
        if self.is.get_char().unwrap() == '.' {
            self.is.next_char();
            return Token::Def(self.read_until("\n;"));
        }

        let next_line = self.read_until("\n:;");

        // 4) Read label
        if let Some(last) = self.is.get_char() {
            if last == ':' {
                self.is.next_char();
                return Token::Label(next_line);
            }
        }

        // 5) Read Instruction
        Token::Ins(next_line)
    }

    fn read_until(&mut self, pattern: &str) -> String {
        let mut res = String::new();
        loop {
            match self.is.get_char() {
                Some(c) if !pattern.contains(c) => res.push(c),
                _ => break,
            }
            self.is.next_char();
        }

        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_file(path: &str) {
        let mut lex = Lexer::from_file(path);
        loop {
            let tok = lex.next();
            println!("{:?}", tok);
            if let Token::EOF = tok {
                break;
            }
        }
    }

    #[test]
    fn fn_add() {
        test_file("../irintsm/tests/fn_add.ir");
    }

    #[test]
    fn fn_fact() {
        test_file("../irintsm/tests/fn_fact.ir");
    }

    #[test]
    fn hello_42() {
        test_file("../irintsm/tests/hello_42.ir");
    }
}
