
# irint3a

The names comes from the fact that's the IR only supports int32, and it's register-based, using 3-adresses instructions (non-destructive instructions, they have different source and destination registers).  
The IR is not typed, and there are an infinite number of registers.  
It's also possible to access memory, using 32-bits addresses stored in registers.

Instructions:
- data: movi, movr, load, store, alloca
- math: add, sub, mul, div, mod
- compareason: cmpeq, cmplt, cmpgt
- control flow: jump, br, call, ret

More details can be found at `src/ir.rs`

## IR structure

A module is a list of functions (declaration or definition).  
Each function definition is a sequence of basic blocks.  
Each basic block is a sequence of instructions.  
It's possible to construct and manipulate an invalid IR, and there is a module to check the validity.
