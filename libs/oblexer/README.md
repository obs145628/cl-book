
Lexer library for compilers.  

Can read data from different kind of inputs, in a transparent manner.
- files
- raw strings

Generate a sequence of token of different kinds:
- identifier
- keyword
- symbol (eg + - & , etc)
- literals: int / string / float

The compiler gives the lexer a list of valid keywords and symbols, and the lexer uses a trie to recognize them efficiently.
