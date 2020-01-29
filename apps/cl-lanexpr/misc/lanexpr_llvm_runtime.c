#include <stdint.h>

int putchar(int c);

void _f1_main();


void _std_putc(int32_t c)
{
  putchar(c);
}

int main()
{
  _f1_main();
  return 0;
}
