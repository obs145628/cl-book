
# irintsm

The names comes from the fact that's the IR is for a stack machine, and only supports i32.  
Each function has it's own stack frame it can manipulate through:
- local variables (with arguments)
- operands stack: all instructions pops inputs and push results there

Instructions:
- operand stacks: pop, const
- local variables: load, store
- math: add, sub, mul, div, mod
- compareason: cmpeq, cmplt, cmpgt
- control flow: jump, br, call, ret

More details can be found at `src/ir.rs`
