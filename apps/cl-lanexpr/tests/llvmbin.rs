use lanexpr::parser;
use lanexpr::translater;
use lanexpr::typecheck;
use obtests::bintest::{TestRunner, UserRunner};
use obtests::utils;

struct LLVMRunner {}
impl UserRunner for LLVMRunner {
    fn run(&self, path: &str, _input_name: Option<String>, input_path: Option<String>) -> Vec<u8> {
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
}

fn test_file(dir: &str, test_name: &str) {
    let ur = LLVMRunner {};
    let tr = TestRunner::new(dir.to_string(), test_name.to_string());
    tr.run(&ur);
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

#[test]
fn llvm_binary_basics_calc() {
    test_file("../../libs/lanexpr/tests/basics", "calc");
}

#[test]
fn llvm_binary_basics_ivec() {
    test_file("../../libs/lanexpr/tests/basics", "ivec");
}

#[test]
fn llvm_binary_algos1_binsearch() {
    test_file("../../libs/lanexpr/tests/algos1", "binsearch");
}

#[test]
fn llvm_binary_algos1_queuell() {
    test_file("../../libs/lanexpr/tests/algos1", "queuell");
}

#[test]
fn llvm_binary_algos1_stack() {
    test_file("../../libs/lanexpr/tests/algos1", "stack");
}

#[test]
fn llvm_binary_algos1_stackfixed() {
    test_file("../../libs/lanexpr/tests/algos1", "stackfixed");
}

#[test]
fn llvm_binary_algos1_stackll() {
    test_file("../../libs/lanexpr/tests/algos1", "stackll");
}

#[test]
fn llvm_binary_algos1_unionfind() {
    test_file("../../libs/lanexpr/tests/algos1", "unionfind");
}

#[test]
fn llvm_binary_algos2_3wquicksort() {
    test_file("../../libs/lanexpr/tests/algos2", "3wquicksort");
}

#[test]
fn llvm_binary_algos2_bumergesort() {
    test_file("../../libs/lanexpr/tests/algos2", "bumergesort");
}

#[test]
fn llvm_binary_algos2_heap() {
    test_file("../../libs/lanexpr/tests/algos2", "heap");
}

#[test]
fn llvm_binary_algos2_heapsort() {
    test_file("../../libs/lanexpr/tests/algos2", "heapsort");
}

#[test]
fn llvm_binary_algos2_insertionsort() {
    test_file("../../libs/lanexpr/tests/algos2", "insertionsort");
}

#[test]
fn llvm_binary_algos2_quicksort() {
    test_file("../../libs/lanexpr/tests/algos2", "quicksort");
}

#[test]
fn llvm_binary_algos2_selectionsort() {
    test_file("../../libs/lanexpr/tests/algos2", "selectionsort");
}

#[test]
fn llvm_binary_algos2_shellsort() {
    test_file("../../libs/lanexpr/tests/algos2", "shellsort");
}

#[test]
fn llvm_binary_algos2_tdmergesort() {
    test_file("../../libs/lanexpr/tests/algos2", "tdmergesort");
}

#[test]
fn llvm_binary_algos3_bsttable() {
    test_file("../../libs/lanexpr/tests/algos3", "bsttable");
}

#[test]
fn llvm_binary_algos3_hashtable() {
    test_file("../../libs/lanexpr/tests/algos3", "hashtable");
}

#[test]
fn llvm_binary_algos3_lltable() {
    test_file("../../libs/lanexpr/tests/algos3", "lltable");
}
