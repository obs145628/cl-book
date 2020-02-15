#ifndef PRINT_H_
#define PRINT_H_

#include "std.h"

void print_rec(int32_t x) {
  if (!(x == 0)) {
    print_rec(x / 10);
    std_putc(48 + (x % 10));
  }
}

void print_int(int32_t x) {
  if (x < 0) {
    std_putc(45);
    print_rec(-x);
  }

  else if (x == 0) {
    std_putc(48);
  } else {
    print_rec(x);
  }
}

void printnl() { std_putc(10); }

void printnl_int(int32_t x) {
  print_int(x);
  printnl();
}

#endif //! PRINT_H_
