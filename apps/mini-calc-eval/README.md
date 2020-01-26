Evaluator for lancalc

# lancalc

It's not really a language, just a simple math expression, with a few features:
- int64 and double data types
- operators +, -, *, /
- parentheses

# Evaluation

There are 2 ways to evaluate the expression:
- Use a parser that directly computes and returns the result.
- Use parser that builds the AST, then evaluates it.
