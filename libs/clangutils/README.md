
Library to create binary files (library, objects, executables) using clang.  
Right now it calls the clang binary.  
It works only if clang is on the path.

A nice improvement would be to use llvm-sys to compile the files, instead of spawning commands.
