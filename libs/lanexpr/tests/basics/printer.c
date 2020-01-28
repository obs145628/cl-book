#include <stdint.h>
#include <stdio.h>

void print_rec(int32_t x)
{
  if (!(x == 0)) {
    print_rec(x / 10);
    putchar(48 + (x % 10));
  }
} 

void print_int(int32_t x)
{
  if (x < 0) {
    putchar(45);
    print_rec(-x);
  }

  else if (x == 0) {
    putchar(48);
  }
  else {
    print_rec(x);
  }
}

void printnl()
{
  putchar(10);
}

void printnl_int(int32_t x)
{
  print_int(x);
  printnl();
}

int main()
{
  printnl_int(18);
  printnl_int(0);
  printnl_int(-6);
  printnl_int(8);
  printnl_int(-456);
}
