use obtests::bintest::{TestRunner, UserRunner};

struct IRRunner {}
impl UserRunner for IRRunner {
    fn run(&self, path: &str, _input_name: Option<String>, input_path: Option<String>) -> Vec<u8> {
        let input_path: Option<&str> = match input_path.as_ref() {
            Some(x) => Some(x),
            None => None,
        };

        // parsing
        let mut ps = lanexpr::parser::Parser::new_from_file(path);
        let root = ps.parse();

        // type-checkinh
        let mut tc = lanexpr::typecheck::TypeCheck::new();
        tc.check(&root);
        let ba = tc.get_bindings();

        // translation
        let tr = lanexpr::translater::irint3a::Translater::new(&root, &ba);

        // execution
        let code = tr.translate().0;
        let mut rt = interp_irint3a::runtime::Runtime::new(code);
        if let Some(input_path) = input_path {
            rt.reset_stdin_path(input_path);
        }
        rt.run();
        Vec::from(rt.stdout())
    }
}

fn test_file(dir: &str, test_name: &str) {
    let ur = IRRunner {};
    let tr = TestRunner::new(dir.to_string(), test_name.to_string());
    tr.run(&ur);
}

#[test]
fn interp_irint3a_basics_printer() {
    test_file("../../libs/lanexpr/tests/basics", "printer");
}

#[test]
fn interp_irint3a_basics_fibo() {
    test_file("../../libs/lanexpr/tests/basics", "fibo");
}

#[test]
fn interp_irint3a_basics_fact() {
    test_file("../../libs/lanexpr/tests/basics", "fact");
}

#[test]
fn interp_irint3a_basics_cat() {
    test_file("../../libs/lanexpr/tests/basics", "cat");
}

#[test]
fn interp_irint3a_basics_calc() {
    test_file("../../libs/lanexpr/tests/basics", "calc");
}

#[test]
fn llvm_binary_basics_ivec() {
    test_file("../../libs/lanexpr/tests/basics", "ivec");
}

#[test]
fn interp_irint3a_algos1_binsearch() {
    test_file("../../libs/lanexpr/tests/algos1", "binsearch");
}

#[test]
fn interp_irint3a_algos1_queuell() {
    test_file("../../libs/lanexpr/tests/algos1", "queuell");
}

#[test]
fn interp_irint3a_algos1_stack() {
    test_file("../../libs/lanexpr/tests/algos1", "stack");
}

#[test]
fn interp_irint3a_algos1_stackfixed() {
    test_file("../../libs/lanexpr/tests/algos1", "stackfixed");
}

#[test]
fn interp_irint3a_algos1_stackll() {
    test_file("../../libs/lanexpr/tests/algos1", "stackll");
}

#[test]
fn interp_irint3a_algos1_unionfind() {
    test_file("../../libs/lanexpr/tests/algos1", "unionfind");
}

#[test]
fn interp_irint3a_algos2_3wquicksort() {
    test_file("../../libs/lanexpr/tests/algos2", "3wquicksort");
}

#[test]
fn interp_irint3a_algos2_bumergesort() {
    test_file("../../libs/lanexpr/tests/algos2", "bumergesort");
}

#[test]
fn interp_irint3a_algos2_heap() {
    test_file("../../libs/lanexpr/tests/algos2", "heap");
}

#[test]
fn interp_irint3a_algos2_heapsort() {
    test_file("../../libs/lanexpr/tests/algos2", "heapsort");
}

#[test]
fn interp_irint3a_algos2_insertionsort() {
    test_file("../../libs/lanexpr/tests/algos2", "insertionsort");
}

#[test]
fn interp_irint3a_algos2_quicksort() {
    test_file("../../libs/lanexpr/tests/algos2", "quicksort");
}

#[test]
fn interp_irint3a_algos2_selectionsort() {
    test_file("../../libs/lanexpr/tests/algos2", "selectionsort");
}

#[test]
fn interp_irint3a_algos2_shellsort() {
    test_file("../../libs/lanexpr/tests/algos2", "shellsort");
}

#[test]
fn interp_irint3a_algos2_tdmergesort() {
    test_file("../../libs/lanexpr/tests/algos2", "tdmergesort");
}

#[test]
fn interp_irint3a_algos3_bsttable() {
    test_file("../../libs/lanexpr/tests/algos3", "bsttable");
}

#[test]
fn llvm_binary_algos3_hashtable() {
    test_file("../../libs/lanexpr/tests/algos3", "hashtable");
}

#[test]
fn interp_irint3a_algos3_lltable() {
    test_file("../../libs/lanexpr/tests/algos3", "lltable");
}
