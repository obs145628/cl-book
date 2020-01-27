
# lanexpr

Programming language where everything is an expression.  
The whole program is actually an expression that gets evaluated as the main function.  
Main features:
- 2 types: int32 and void
- constructs: if/else, while, sequence, scope, function definitons, variable definitions, assignment
- binary operators: +, -, *, /, %, ==, <, >
- unary operators: +, -, !
- constants, variables, function calls

# Compilation

## Parser

The parser check the code syntax and build an AST

## Typecheck

Perform semantic analysis: check if the code is correct, and generate symbol tables for all the definitions and use in the code.

## IR generation

Lower the AST into an IR representation.  
Can be converted into multiple representations:
- irint3a


# Standard library

Basic native functions avalaible in the language:
 - putc(int32): write 1 char on the standard output.
