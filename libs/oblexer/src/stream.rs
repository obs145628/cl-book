use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct Stream {
    buf: std::io::BufReader<std::fs::File>,
    line: Vec<char>,
    pos: usize,
    is_eof: bool,
}

impl Stream {

    pub fn new(path: &str) -> Stream {
	let f = File::open(path).unwrap();
	let mut res = Stream {
	    buf: BufReader::new(f),
	    line: Vec::new(),
	    pos: 0,
	    is_eof: false
	};
	res.load_char();
	res
    }

    pub fn eof(&self) -> bool {
	return self.is_eof;
    }

    pub fn get_char(&self) -> char {
	if self.is_eof {
	    panic!("Stream::get_char(): EOF reached");
	}
	self.line[self.pos]
    }

    pub fn next_char(&mut self) {
	self.pos += 1;
	self.load_char();
    }

    fn load_char(&mut self) {
	if self.pos == self.line.len() {
	    let mut str_line = String::new();
	    let nread = self.buf.read_line(&mut str_line).unwrap();
	    self.line = str_line.chars().collect();
	    self.is_eof = nread == 0;
	    self.pos = 0;
	}
    }
}
