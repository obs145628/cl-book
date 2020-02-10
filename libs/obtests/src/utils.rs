use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

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
