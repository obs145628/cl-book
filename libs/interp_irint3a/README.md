
Interpreter for irint3a modules.

This is a really basic interpreter, without any optimizations, to be able to test the compilers.  
It implements all the required native functions to run any source code.  
The program output is stored into a bytes array.

# Standard functions

The followings functions are implemented with respect to LExpr reference:
- putc
- getc (TODO)
- exit
- fmemget (TODO)
- fmemset (TODO)
- fmemcpy (TODO)


# Execution flow

The program starts by running the function 0 without any parameters.  
The only way to stop the program is by calling exit.  
Returning from the function 0 panics the interpreter.

