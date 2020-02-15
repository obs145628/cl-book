#ifndef DEBUG_H_
#define DEBUG_H_

#include "std.h"

void panic() {
  std_putc(80);
  std_putc(65);
  std_putc(78);
  std_putc(73);
  std_putc(67);
  std_putc(33);
  std_putc(10);
  std_exit(1);
}

// Panic if `cond` is false
void panic_ifn(int cond) {
  if (!cond)
    panic();
}

#endif //! DEBUG_H_
