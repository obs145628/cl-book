use std::path::Path;

use lanexpr::parser;
use lanexpr::translater;
use lanexpr::typecheck;

const FORCE_REBUILD: bool = false;

mod utils {
    use std::collections::hash_map::DefaultHasher;
    use std::fs::File;
    use std::hash::Hash;
    use std::hash::Hasher;
    use std::io::Read;
    use std::io::Write;
    use std::process::Command;
    use std::process::Stdio;

    pub fn run_cmd<I, S>(name: &str, args: I, stdin_path: Option<&str>) -> Vec<u8>
    where
        I: IntoIterator<Item = S> + std::fmt::Debug,
        S: AsRef<std::ffi::OsStr>,
    {
        match stdin_path {
            Some(stdin_path) => println!("Run command '{} {:?} < {}", name, args, stdin_path),
            None => println!("Run command '{} {:?}", name, args),
        }

        let stdin: Stdio = match stdin_path {
            Some(stdin_path) => {
                Stdio::from(File::open(stdin_path).expect("Failed to open input file for stdin"))
            }
            None => Stdio::null(),
        };

        let cmd = Command::new(name)
            .args(args)
            .stdin(stdin)
            .output()
            .expect("Failed to run command");

        if !cmd.status.success() {
            println!("Command failed with exit status {}", cmd.status);
            println!("OUTPUT: <BEG>{}<END>", String::from_utf8_lossy(&cmd.stdout));
            println!(" ERROR: <BEG>{}<END>", String::from_utf8_lossy(&cmd.stderr));
            panic!("Error in the testsuite");
        }

        cmd.stdout
    }

    pub fn read_file_bin(path: &str) -> Vec<u8> {
        println!("Read binary file {}", path);
        let mut f = File::open(path).expect("Failed to open file");
        let mut res = vec![];
        f.read_to_end(&mut res).expect("Failed to read file");
        res
    }

    pub fn write_file_bin(path: &str, data: &[u8]) {
        println!("Write binary file {} ({}o)", path, data.len());
        let mut f = File::create(path).expect("Failed to create file");
        f.write_all(data).expect("Failed to write  file");
    }

    pub fn calculate_hash(t: &str) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

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

struct TestRunner {
    dir: String,
    test_name: String,
}

impl TestRunner {
    pub fn new(dir: String, test_name: String) -> Self {
        TestRunner { dir, test_name }
    }

    pub fn run(&self) {
        let inputs = self.list_inputs();
        if inputs.len() == 0 {
            return self.run_one(None);
        }

        for in_name in inputs {
            self.run_one(Some(in_name));
        }
    }

    fn src_path(&self) -> String {
        let path = Path::new(&self.dir).join(self.test_name.clone() + ".le");
        path.to_str().unwrap().to_string()
    }

    fn get_llvm_output(&self, path: &str, input_name: Option<String>) -> Vec<u8> {
        let input_path = self.input_path(input_name);
        let input_path: Option<&str> = match input_path.as_ref() {
            Some(x) => Some(x),
            None => None,
        };
        let mut ps = parser::Parser::new_from_file(path);
        let ast = ps.parse();

        let mut tc = typecheck::TypeCheck::new();
        tc.check(&ast);
        let ba = tc.get_bindings();

        let tmp_bin_path = format!(
            "/tmp/lanexpr_llvm_bin_tmp_{}.out",
            utils::calculate_hash(path)
        );
        translater::llvmbin::compile_to_binary(&ast, &ba, &tmp_bin_path);
        let res = utils::run_cmd(&tmp_bin_path, &[] as &[&str], input_path);
        std::fs::remove_file(&tmp_bin_path).expect("Failed to remove temporary bin file");
        res
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

    fn run_one(&self, input_name: Option<String>) {
        match &input_name {
            Some(in_name) => println!("Running test {} < {}", self.test_name, in_name),
            None => println!("Running test {}", self.test_name),
        }

        let rb = RefBuilder::new(self.dir.clone(), self.test_name.clone(), input_name.clone());
        rb.prepare();
        let ref_out = rb.get_ref();

        let src_path = self.src_path();
        let code_out = self.get_llvm_output(&src_path, input_name);
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

fn test_file(dir: &str, test_name: &str) {
    let tr = TestRunner::new(dir.to_string(), test_name.to_string());
    tr.run();
}

#[test]
fn llvm_binary_basics_printer() {
    test_file("../../libs/lanexpr/tests/basics", "printer");
}

#[test]
fn llvm_binary_basics_fibo() {
    test_file("../../libs/lanexpr/tests/basics", "fibo");
}

#[test]
fn llvm_binary_basics_fact() {
    test_file("../../libs/lanexpr/tests/basics", "fact");
}

#[test]
fn llvm_binary_basics_cat() {
    test_file("../../libs/lanexpr/tests/basics", "cat");
}
