pub mod runtime;

#[cfg(test)]
mod tests {

    use super::*;

    pub fn run_file(path: &str, expected: &str) {
        let ps = irint3a::irparser::Parser::from_file(path);
        let (module, _name) = ps.build();

        let mut rt = runtime::Runtime::new(module);
        rt.run();
        let out = std::str::from_utf8(rt.stdout()).expect("Non UTF-8 chars in program output");
        assert_eq!(out, expected);
    }

    #[test]
    fn run_hello_42() {
        run_file("../irint3a/tests/hello_42.ir", "42\n");
    }
}
