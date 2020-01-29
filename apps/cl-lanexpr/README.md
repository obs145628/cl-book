Compiler for lanexpr.  

Can perform many operations:
- parse a file and build the AST
- typecheck a file
- build and print the symbol tables from a file
- convert the input to irint3a and print the IR
- convert the input to LLVM IR
- compile to a standalone binary (using LLVM IR)

For more details:
```shell
cargo run -- --help
```
