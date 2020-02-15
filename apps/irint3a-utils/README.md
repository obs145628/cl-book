
Utils to manipulate IR files of irint3a.  
It reads an input IR file, parse it, validates it, and can do one the following:
- print back the parsed IR
- run the IR with an interpreter
- run analysis and print some output / graph infos

# Usage

```shell
cargo run <input-file> [options]
```

For more infos:

```shell
cargo run -- --help
```
# Example : Display CFG Graph

It's possible to display the control flow graph of a function using dot.  
Example from `libs/lanexpr/tests/algos3/bsttable.le` compiled to `bsttable.ir`.  

```shell
cargo run -- bsttable.ir --dump-cfg node_del_19
dot -Tpng cfg.dot -o cfg.png
feh cfg.png
```
