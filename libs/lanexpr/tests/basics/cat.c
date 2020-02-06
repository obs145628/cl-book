#include <stdio.h>

int main() {
  int val = 0;
  while (val != -1) {
    val = getchar();
    if (val != -1)
      putchar(val);
  }
}
