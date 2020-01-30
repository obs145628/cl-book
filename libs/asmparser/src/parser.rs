use crate::lexer::{Lexer, Token};

/// Parsed argument of an instruction
#[derive(Debug)]
pub enum InsArg {
    Const(u64),     // <uint>
    IdInt(u64),     // %<uint>
    IdName(String), // %<str>
}

/// Parsed label of an instruction
#[derive(Debug)]
pub enum InsLabel {
    IdInt(u64),     // <uint>:
    IdName(String), // <str>:
}

/// Parsed instruction
#[derive(Debug)]
pub struct Ins {
    pub label: Option<InsLabel>,
    pub name: String,
    pub args: Vec<InsArg>,
    pub comments: Vec<String>,
}

/// Parsed definition
#[derive(Debug)]
pub struct Def {
    pub line: String,
    pub comments: Vec<String>,
}

/// The object produced by the paser
#[derive(Debug)]
pub enum Item {
    Ins(Ins),
    Def(Def),
}

/// Parser for IR/ASM code
/// Produce a stream of Item
pub struct Parser {
    lex: Lexer,
    next: Option<Item>,
    is_eof: bool,
}

impl Parser {
    pub fn from_file(path: &str) -> Self {
        Parser::new(Lexer::from_file(path))
    }

    pub fn from_str(path: &str) -> Self {
        Parser::new(Lexer::from_str(path))
    }

    pub fn peek(&mut self) -> Option<&Item> {
        self.load_next();
        self.next.as_ref()
    }

    pub fn next(&mut self) -> Option<Item> {
        self.load_next();
        let mut res = None;
        std::mem::swap(&mut res, &mut self.next);
        res
    }

    fn load_next(&mut self) {
        if self.next.is_none() && !self.is_eof {
            self.next = self.read_next();
            self.is_eof = self.next.is_none()
        }
    }

    fn read_next(&mut self) -> Option<Item> {
        // 1) skip first comments and stop at eof
        let main = loop {
            let tok = self.lex.next();
            match tok {
                Token::Comment(_) => {}
                _ => break tok,
            }
        };

        match main {
            Token::Comment(_) => unreachable!(),
            Token::EOF => None,

            // 2) parse def
            Token::Def(def) => self.next_def(def),

            // 3) parse instruction
            Token::Ins(ins) => self.next_ins(ins, None, vec![]),

            // 4) parse label
            Token::Label(label) => self.next_label(label),
        }
    }

    fn next_label(&mut self, label: String) -> Option<Item> {
        let mut comments = vec![];
        let main = loop {
            let tok = self.lex.next();
            match tok {
                Token::Ins(ins) => break ins,
                Token::Comment(com) => comments.push(com),
                Token::Label(_) => panic!("An instruction can only have one label"),
                Token::Def(_) => panic!("A definition cannot have a label"),
                Token::EOF => panic!("End Of File after label"),
            }
        };
        self.next_ins(main, Some(label), comments)
    }

    fn next_def(&mut self, def: String) -> Option<Item> {
        let comments = self.parse_comments();

        Some(Item::Def(Def {
            line: def,
            comments,
        }))
    }

    fn next_ins(
        &mut self,
        ins: String,
        label: Option<String>,
        comments: Vec<String>,
    ) -> Option<Item> {
        let mut comments = comments;
        comments.append(&mut self.parse_comments());

        let label = self.parse_label(label);
        let (name, args) = self.parse_ins(ins);

        Some(Item::Ins(Ins {
            label,
            name,
            args,
            comments,
        }))
    }

    fn parse_label(&self, name: Option<String>) -> Option<InsLabel> {
        let name = name?;
        Some(match name.parse::<u64>() {
            Ok(val) => InsLabel::IdInt(val),
            Err(_) => InsLabel::IdName(name),
        })
    }

    fn parse_ins_arg(&self, arg: &str) -> InsArg {
        let firstc = arg.chars().next().expect("Empty instruction argument");
        let is_id = firstc == '%';
        if !is_id {
            return InsArg::Const(arg.parse().expect("Invalid argument"));
        }

        let id_val = &arg[1..];
        match id_val.parse::<u64>() {
            Ok(val) => InsArg::IdInt(val),
            Err(_) => InsArg::IdName(id_val.to_string()),
        }
    }

    fn parse_ins(&self, line: String) -> (String, Vec<InsArg>) {
        let sline = line.as_str();
        let name_end = line.find(' ');
        if name_end.is_none() {
            return (line, vec![]);
        }
        let name = &sline[0..name_end.unwrap()];
        let args: Vec<_> = sline[name_end.unwrap()..]
            .split(',')
            .map(|x| self.parse_ins_arg(x.trim()))
            .collect();

        (name.to_string(), args)
    }

    fn parse_comments(&mut self) -> Vec<String> {
        let mut comments = vec![];
        loop {
            match self.lex.peek() {
                Token::Comment(com) => comments.push(com.clone()),
                _ => break,
            }
            self.lex.next();
        }
        comments
    }

    fn new(lex: Lexer) -> Self {
        Parser {
            lex,
            next: None,
            is_eof: false,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_file(path: &str) {
        let mut ps = Parser::from_file(path);
        while let Some(it) = ps.next() {
            println!("{:?}", it);
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
