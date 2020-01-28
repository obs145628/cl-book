
Interpreter for irint3a modules.

This is a really basic interpreter, without any optimizations, to be able to test the compilers.  
It implements all the required native functions to run any source code.  
The program output is stored into a bytes array.

# Standard functions

There a few extern functions that are implemented by the interperter to be able to run programs:

- putc(byte_val) (257): write a byte to the standard output

- exit(ret_code) (258): exit the program with the specified return code


# Execution flow

The program starts by running the function 0 without any parameters.  
The only way to stop the program is by calling exit.  
Returning from the function 0 panics the interpreter.
