#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"

int_t stack_new(int_t cap) {
  panic_ifn(cap > 0);
  int_t stack = fm_alloc(cap + 2);
  std_fmemset(stack, cap);
  std_fmemset(stack + 1, 0);
  return stack;
}

void stack_free(int_t stack) { fm_free(stack); }

// Push `val` to the top of the stack
void stack_push(int_t stack, int_t val) {
  int_t cap = std_fmemget(stack);
  int_t len = std_fmemget(stack + 1);
  panic_ifn(len < cap);
  std_fmemset(stack + 1, len + 1);
  std_fmemset(stack + 2 + len, val);
}

// Pop and return one item from the stack
int_t stack_pop(int_t stack) {
  int_t len = std_fmemget(stack + 1);
  panic_ifn(len > 0);
  std_fmemset(stack + 1, len - 1);
  return std_fmemget(stack + 1 + len);
}

int_t stack_size(int_t stack) { return std_fmemget(stack + 1); }

void test1() {
  int_t s = stack_new(16);
  printnl_int(stack_size(s));
  stack_push(s, 16);
  stack_push(s, 14);
  stack_push(s, 8);
  stack_push(s, 7);
  printnl_int(stack_size(s));

  while (stack_size(s)) {
    printnl_int(stack_pop(s));
  }
  stack_free(s);
}

void test2() {
  int_t s = stack_new(1000);
  int_t i = 0;
  while (i < 1000) {
    stack_push(s, 2 * i * i - 12 * i + 6);
    i = i + 1;
  }

  while (stack_size(s)) {
    printnl_int(stack_pop(s));
  }
  stack_free(s);
}

int main() {
  test1();
  test2();
}
