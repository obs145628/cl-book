use std::path::Path;

use crate::utils;

const FORCE_REBUILD: bool = false;

// full src_path: dir + test_name + src_ext
struct RefBuilder {
    dir: String,
    test_name: String,
    input_name: Option<String>,
    src_ext: Option<String>,     //execution of src ref python (python or c)
    tmp_ref_bin: Option<String>, //path to binary file (in case of compiled ref)
}

impl RefBuilder {
    pub fn new(dir: String, test_name: String, input_name: Option<String>) -> Self {
        let (src_ext, tmp_ref_bin) = {
            let src_c_path = Path::new(&dir).join(test_name.clone() + ".c");
            let src_py_path = Path::new(&dir).join(test_name.clone() + ".py");

            if src_c_path.exists() {
                (
                    ".c".to_string(),
                    Some(format!("/tmp/lanexpr_ref_bin_{}", test_name)),
                )
            } else if src_py_path.exists() {
                (".py".to_string(), None)
            } else {
                panic!(
                    "No file found to build ref for test {} in dir {}",
                    test_name, dir
                );
            }
        };

        RefBuilder {
            dir,
            test_name,
            input_name,
            src_ext: Some(src_ext),
            tmp_ref_bin,
        }
    }

    /// Build binary file if needed
    pub fn prepare(&self) {
        match &self.src_ext {
            Some(ext) if ext == ".c" => self.prepare_cpp(),
            Some(ext) if ext == ".py" => self.prepare_py(),
            _ => unreachable!(),
        }
    }

    /// Build the ref file if necessary, then read and returns its content
    pub fn get_ref(&self) -> Vec<u8> {
        let ref_path = self.ref_path();
        self.build_ref();
        utils::read_file_bin(&ref_path)
    }

    fn prepare_cpp(&self) {
        let ref_bin = self.tmp_ref_bin.as_ref().unwrap();
        if !FORCE_REBUILD && Path::new(ref_bin).exists() {
            return;
        }

        let src_path = self.src_ref_path();
        utils::run_cmd("gcc", &[&src_path, "-o", ref_bin], None);
    }

    fn prepare_py(&self) {}

    fn src_ref_path(&self) -> String {
        let path =
            Path::new(&self.dir).join(self.test_name.clone() + self.src_ext.as_ref().unwrap());
        path.to_str().unwrap().to_string()
    }

    fn ref_path(&self) -> String {
        let path = match self.input_name.as_ref() {
            Some(in_name) => {
                Path::new(&self.dir).join(format!("{}_{}.out", self.test_name, in_name))
            }
            None => Path::new(&self.dir).join(self.test_name.clone() + ".out"),
        };
        path.to_str().unwrap().to_string()
    }

    fn input_path(&self) -> Option<String> {
        let in_name = self.input_name.as_ref()?;
        let path = Path::new(&self.dir)
            .join(format!("input_{}", self.test_name))
            .join(in_name);
        Some(path.to_str().unwrap().to_string())
    }

    fn run_cmd(&self) -> Vec<u8> {
        let input_path = self.input_path();
        let input_path: Option<&str> = match input_path.as_ref() {
            Some(x) => Some(x),
            None => None,
        };

        match &self.src_ext {
            Some(ex) if ex == ".c" => {
                let ref_bin = self.tmp_ref_bin.as_ref().unwrap();
                utils::run_cmd(ref_bin, &[] as &[&str], input_path)
            }
            Some(ex) if ex == ".py" => {
                let src_path = self.src_ref_path();
                utils::run_cmd("python3", &[src_path], input_path)
            }
            _ => unreachable!(),
        }
    }

    fn build_ref(&self) {
        let ref_path = self.ref_path();
        if !FORCE_REBUILD && Path::new(&ref_path).is_file() {
            return;
        }

        let out = self.run_cmd();
        utils::write_file_bin(&ref_path, &out);
    }
}

pub trait UserRunner {
    fn run(&self, path: &str, input_name: Option<String>, input_path: Option<String>) -> Vec<u8>;
}

pub struct TestRunner {
    dir: String,
    test_name: String,
}

impl TestRunner {
    pub fn new(dir: String, test_name: String) -> Self {
        TestRunner { dir, test_name }
    }

    pub fn run(&self, ur: &dyn UserRunner) {
        let inputs = self.list_inputs();
        if inputs.len() == 0 {
            return self.run_one(ur, None);
        }

        for in_name in inputs {
            self.run_one(ur, Some(in_name));
        }
    }

    fn src_path(&self) -> String {
        let path = Path::new(&self.dir).join(self.test_name.clone() + ".le");
        path.to_str().unwrap().to_string()
    }

    fn check_output(&self, code_out: &[u8], ref_out: &[u8]) {
        if code_out == ref_out {
            return;
        }

        println!("Test {} failed: output differs", self.test_name);
        println!("Expected:\n><BEG>{}<END>", String::from_utf8_lossy(ref_out));
        println!(" Actual:\n><BEG>{}<END>", String::from_utf8_lossy(code_out));
        panic!("Test failed");
    }

    fn run_one(&self, ur: &dyn UserRunner, input_name: Option<String>) {
        match &input_name {
            Some(in_name) => println!("Running test {} < {}", self.test_name, in_name),
            None => println!("Running test {}", self.test_name),
        }

        let rb = RefBuilder::new(self.dir.clone(), self.test_name.clone(), input_name.clone());
        rb.prepare();
        let ref_out = rb.get_ref();

        let src_path = self.src_path();
        let input_path = self.input_path(input_name.clone());
        let code_out = ur.run(&src_path, input_name, input_path);
        self.check_output(&code_out, &ref_out);
    }

    fn list_inputs(&self) -> Vec<String> {
        let input_dir = Path::new(&self.dir).join(format!("input_{}", self.test_name));
        if !input_dir.is_dir() {
            return vec![];
        }

        std::fs::read_dir(input_dir)
            .expect("Failed to read inputs dir")
            .map(|e| {
                e.unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect()
    }

    fn input_path(&self, in_name: Option<String>) -> Option<String> {
        let in_name = in_name?;
        let path = Path::new(&self.dir)
            .join(format!("input_{}", self.test_name))
            .join(in_name);
        Some(path.to_str().unwrap().to_string())
    }
}
