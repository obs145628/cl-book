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

struct StringStreamLine {
    data: String,
    pos: usize,
}

impl StringStreamLine {
    pub fn new(data: &str) -> StringStreamLine {
        StringStreamLine {
            data: String::from(data),
            pos: 0,
        }
    }
}

impl StreamLine for StringStreamLine {
    fn next_line(&mut self) -> Option<String> {
        if self.pos == self.data.len() {
            return None;
        }

        match &self.data[self.pos..].find('\n') {
            Some(new_pos) => {
                let new_pos = self.pos + *new_pos;
                let res = String::from(&self.data[self.pos..new_pos + 1]);
                self.pos = new_pos + 1;
                Some(res)
            }

            None => {
                let res = String::from(&self.data[self.pos..]);
                self.pos = self.data.len();
                Some(res)
            }
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

    pub fn from_str(data: &str) -> Stream {
        let mut res = Stream {
            is: Box::new(StringStreamLine::new(data)),
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

    pub fn get_char(&self) -> Option<char> {
        if self.is_eof {
            None
        } else {
            Some(self.line[self.pos])
        }
    }

    pub fn next_char(&mut self) {
        self.pos += 1;
        self.load_char();
    }

    fn load_char(&mut self) {
        if self.pos == self.line.len() {
            self.pos = 0;
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

#[cfg(test)]
mod tests {

    use super::*;

    fn read_files_lines(path: &str) -> Vec<String> {
        let f = File::open(path).unwrap();
        let is = std::io::BufReader::new(f);
        let mut res: Vec<String> = is.lines().map(|l| l.unwrap().to_string() + "\n").collect();

        while res.len() > 1 && res.last().unwrap().len() == 1 {
            res.pop();
        }
        res
    }

    fn read_lines_filestream(path: &str) -> Vec<String> {
        let mut res = vec![];
        let mut is = FileStreamLine::new(path);
        loop {
            match is.next_line() {
                Some(l) => res.push(l),
                None => break,
            }
        }

        while res.len() > 1 && res.last().unwrap().len() == 1 {
            res.pop();
        }
        res
    }

    fn read_str_lines(s: &str) -> Vec<String> {
        let mut res: Vec<String> = s.split("\n").map(|l| l.to_string() + "\n").collect();

        while res.len() > 1 && res.last().unwrap().len() == 1 {
            res.pop();
        }
        res
    }

    fn read_lines_strstream(s: &str) -> Vec<String> {
        let mut res = vec![];
        let mut is = StringStreamLine::new(s);
        loop {
            match is.next_line() {
                Some(l) => res.push(l),
                None => break,
            }
        }

        while res.len() > 1 && res.last().unwrap().len() == 1 {
            res.pop();
        }
        res
    }

    fn test_read_lines_file(path: &str) {
        let the_ref = read_files_lines(path);
        let my_res = read_lines_filestream(path);
        assert_eq!(the_ref, my_res);
    }

    fn test_read_lines_str(path: &str) {
        let s = std::fs::read_to_string(path).unwrap();
        let the_ref = read_str_lines(&s);
        let my_res = read_lines_strstream(&s);
        assert_eq!(the_ref, my_res);
    }

    #[test]
    fn test_read_lines_f1() {
        test_read_lines_file("./tests/lines1.txt");
    }

    #[test]
    fn test_read_lines_oneline() {
        test_read_lines_file("./tests/oneline.txt");
    }

    #[test]
    fn test_read_lines_empty() {
        test_read_lines_file("./tests/empty.txt");
    }

    #[test]
    fn test_read_lines_f2() {
        test_read_lines_file("./tests/lines2.txt");
    }

    #[test]
    fn test_read_lines_f1_str() {
        test_read_lines_str("./tests/lines1.txt");
    }

    #[test]
    fn test_read_lines_f2_str() {
        test_read_lines_str("./tests/lines2.txt");
    }

    #[test]
    fn test_read_lines_oneline_str() {
        test_read_lines_str("./tests/oneline.txt");
    }
}
