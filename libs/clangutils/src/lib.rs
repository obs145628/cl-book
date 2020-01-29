use std::process::Command;

pub enum ClangOutputType {
    OBJECT,
    BINARY,
}

pub struct ClangCommandBuilder {
    in_files: Option<Vec<String>>,
    out_file: Option<String>,
    out_type: Option<ClangOutputType>,
    clang_bin: &'static str,
}

impl ClangCommandBuilder {
    pub fn new() -> Self {
        ClangCommandBuilder {
            in_files: None,
            out_file: None,
            out_type: None,
            clang_bin: "clang",
        }
    }

    pub fn set_input(self, path: &str) -> Self {
        self.set_inputs(&[path])
    }

    pub fn set_inputs(mut self, paths: &[&str]) -> Self {
        self.in_files = None;
        for path in paths {
            self.m_add_input(path);
        }
        self
    }

    pub fn add_input(mut self, path: &str) -> Self {
        self.m_add_input(path);
        self
    }

    pub fn set_output(mut self, path: &str) -> Self {
        self.out_file = Some(path.to_string());
        self
    }

    pub fn set_output_type(mut self, out_type: ClangOutputType) -> Self {
        self.out_type = Some(out_type);
        self
    }

    pub fn run(self) {
        let in_files = self.in_files.expect("Missing input file");
        let out_file = self.out_file.expect("Imissing output file");
        let out_type = self.out_type.expect("Missing output type");

        let mut args: Vec<&str> = vec![];
        for f in &in_files {
            args.push(f);
        }

        args.push("-o");
        args.push(&out_file);

        match out_type {
            ClangOutputType::OBJECT => args.push("-c"),
            ClangOutputType::BINARY => {}
        }

        Command::new(self.clang_bin)
            .args(&args)
            .output()
            .expect("Command failed");
    }

    fn m_add_input(&mut self, path: &str) {
        if self.in_files.is_none() {
            self.in_files = Some(vec![]);
        }
        self.in_files.as_mut().unwrap().push(path.to_string());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
