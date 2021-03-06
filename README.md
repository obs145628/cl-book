This repo contains a lot of libraries and projects related to compilers.  
It's based on my readings of [Engineering a Compiler, 2nd Edition](https://www.elsevier.com/books/engineering-a-compiler/cooper/978-0-12-088478-0).  
Everything is written in Rust.  
My goal is to write multiple front-end, middle-end, and back-end representations for different languages and architectures.  
These representations will be a lot simpler and with much less features than for a true compiler, as I only do this to learn compilers.  

# Requirements

- Rust (Tested with rustc 1.40.0)
- LLVM (Tested with v9.0.0) (some compilers generate LLVM IR).

Tested on Ubuntu 18.04

# Front-End languages

## lancalc

A math expression with int, float, 4 operators and parentheses.  
Performs evaluation with or without building an AST.

Projects:
- `./app/mini-calc-eval`.

## LExpr (lanexpr)

Language with only int32 type, where everything is a value. Can do functions, variables, scopes, if, loops.

Projects:
- `./libs/lanexpr`
- `./apps/cl-lanexpr`


# Intermediate Representations

## irint3a

Register-based IR with int32 instructions and 32bits pointers. Can do basic maths, functions, calls, and (un)conditional branching.

Projects:
- `./libs/irint3a`
- `./apps/irint3a-utils`

## irintsm

Really similar to irint3a, except that it's based on a stack-machine and doesn't have any registers

Projects:
- `./libs/irintsm`
- `./apps/irintsm-utils`

## LLVM IR

Some compilers generate LLVM IR code.

Projects:
- `./libs/lanexpr`


# Testing

This script build and test all projects:

```shell
./check.sh
```

You can just do the building by using --only-build
