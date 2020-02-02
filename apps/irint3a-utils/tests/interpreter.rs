use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use interp_irint3a::runtime;
use irint3a::ir;

fn calculate_hash(t: &str) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn build_ref_file(path: &str) -> PathBuf {
    let ref_path = Path::new(path).with_extension("out");
    if ref_path.exists() {
        return ref_path;
    }

    let c_path = Path::new(path).with_extension("c");
    let tmp_bin_path = format!("/tmp/cl_bin_tmp_{}.out", calculate_hash(path));

    let _out = Command::new("gcc")
        .args(&[c_path.to_str().unwrap(), "-o", &tmp_bin_path])
        .output()
        .expect(&format!(
            "Failed to compile C file {} to build ref for {}",
            c_path.to_str().unwrap(),
            path,
        ));

    let c_out = Command::new(&tmp_bin_path).output().expect(&format!(
        "Failed to run bin file compiled from C file {} to build ref for {}",
        c_path.to_str().unwrap(),
        path,
    ));

    std::fs::remove_file(&tmp_bin_path).expect("Failed to remove temporary bin file");

    let mut ref_file = File::create(ref_path.clone()).expect("Failed to create ref file");
    ref_file
        .write_all(&c_out.stdout)
        .expect("Failed to write to ref file");

    ref_path
}

fn read_ref_file(path: &str) -> Vec<u8> {
    let ref_path = build_ref_file(path);
    let mut ref_file = File::open(ref_path).expect("Failed to open ref file");
    let mut res = vec![];
    ref_file
        .read_to_end(&mut res)
        .expect("Failed to read ref file");
    res
}

fn compile_lanexpr(path: &str) -> ir::Module {
    let mut ps = lanexpr::parser::Parser::new_from_file(path);
    let root = ps.parse();

    let mut tc = lanexpr::typecheck::TypeCheck::new();
    tc.check(&root);
    let ba = tc.get_bindings();

    let tr = lanexpr::translater::irint3a::Translater::new(&root, &ba);
    tr.translate().0
}

fn exec_lanexpr_file(path: &str) -> Vec<u8> {
    let code = compile_lanexpr(path);
    let mut rt = runtime::Runtime::new(code);
    rt.run();
    Vec::from(rt.stdout())
}

fn test_lanexpr(path: &str) {
    let ref_bytes = read_ref_file(path);
    let out_bytes = exec_lanexpr_file(path);

    println!("REF:\n<BEG>{}<END>", String::from_utf8_lossy(&ref_bytes));
    println!(" ME:\n<BEG>{}<END>", String::from_utf8_lossy(&out_bytes));
    assert_eq!(ref_bytes, out_bytes);
}

#[test]
fn interpret_basics_printer() {
    test_lanexpr("../../libs/lanexpr/tests/basics/printer.le");
}

#[test]
fn interpret_basics_fact() {
    test_lanexpr("../../libs/lanexpr/tests/basics/fact.le");
}

#[test]
fn interpret_basics_fibo() {
    test_lanexpr("../../libs/lanexpr/tests/basics/fibo.le");
}
