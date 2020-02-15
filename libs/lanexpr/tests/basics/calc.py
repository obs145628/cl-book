import sys

in_str = sys.stdin.read().replace('/', '//')
res = eval(in_str)
print(res)
