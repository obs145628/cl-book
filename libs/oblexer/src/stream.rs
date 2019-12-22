use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

trait StreamLine {
    fn next_line(&mut self) -> Option<String>;
}

struct FileStreamLine {
    buf: std::io::BufReader<std::fs::File>,
    is_eof: bool,
}

impl FileStreamLine {
    pub fn new(path: &str) -> FileStreamLine {
        FileStreamLine {
            buf: BufReader::new(File::open(path).unwrap()),
            is_eof: false,
        }
    }
}

impl StreamLine for FileStreamLine {
    fn next_line(&mut self) -> Option<String> {
        if self.is_eof {
            return None;
        }

        let mut str_line = String::new();
        let nread = self.buf.read_line(&mut str_line).unwrap();
        self.is_eof = nread == 0;

        if self.is_eof {
            None
        } else {
            Some(str_line)
        }
    }
}

pub struct Stream {
    is: Box<dyn StreamLine>,
    line: Vec<char>,
    pos: usize,
    is_eof: bool,
}

impl Stream {
    pub fn from_file(path: &str) -> Stream {
        let mut res = Stream {
            is: Box::new(FileStreamLine::new(path)),
            line: Vec::new(),
            pos: 0,
            is_eof: false,
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
            match self.is.next_line() {
                Some(str_line) => {
                    self.line = str_line.chars().collect();
                    self.is_eof = false;
                }

                None => {
                    self.line = vec![];
                    self.is_eof = true;
                }
            }
        }
    }
}
