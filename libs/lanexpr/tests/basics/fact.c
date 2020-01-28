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


int32_t fact_rec(int32_t x)
{
  return x < 2 ? 1 : x * fact_rec(x - 1);
}

int32_t fact_iter(int32_t x)
{
  int32_t res = 1;
  while (x > 1) {
    res = res * x;
    x = x - 1;
  }
  return res;
}

int main()
{
  int32_t i = 0;
  while (i < 10) {
    print_int(i);
    putchar(58);
    putchar(32);
    print_int(fact_rec(i));
    putchar(44);
    putchar(32);
    printnl_int(fact_iter(i));
    i = i + 1;
  }
}
