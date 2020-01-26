This repo contains a lot of libraries and projects related to compilers.  
It's based on my readings of [Engineering a Compiler, 2nd Edition](https://www.elsevier.com/books/engineering-a-compiler/cooper/978-0-12-088478-0).  
Everything is written in Rust.  
My goal is to write multiple front-end, middle-end, and back-end representations for different languages and architectures.  
These representations will be a lot simpler and with much less features than for a true compiler, as I only do this to learn compilers.  

# Requirements

Only Rust.  
Tested with rustc 1.40.0 on Ubuntu 18.04

# Front-End languages

## lancalc

A math expression with int, float, 4 operators and parentheses.  
Performs evaluation with or without building an AST.

Projects:
- `./app/mini-calc-eval`.

## lanexpr

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


# Testing

This script build and test all projects:

```shell
./check.sh
```
