#include <stdint.h>
#include <stddef.h>

// ==== STANDARD DECLARATIONS ====

typedef int32_t int_t;

#define STD_FMEM_SIZE (16 * 1024 * 1024)



// Write one byte to the standard output
void _std_putc(int_t byte_val);

// Read one byte from the standard input
int_t _std_getc();

// Exit the program with return code ret_code
void _std_exit(int_t ret_code);

// Read the flat memory 32b entry at index pos
int_t _std_fmemget(int_t pos);

// Write the flat memory 32b entry at index pos
void _std_fmemset(int_t pos, int_t val);

// Copy n entries starting at index src, to the n entries starting at index dst
// src and dst can overlap
void _std_fmemcpy(int_t dst, int_t src, int_t n);


// ====== STANDARD DEFINITIOND ======

int getchar();
int putchar(int);
void exit(int);

void *memmove(void *dst, const void *src, size_t n);

void *malloc(size_t);

static void std_check(int val, const char *mess) {
  if (val)
    return;
  for (; *mess; ++mess)
    putchar(*mess);
  putchar('\n');
  exit(26);
}

void _std_putc(int_t byte_val) { putchar(byte_val); }

int_t _std_getc() { return getchar(); }

void _std_exit(int_t ret_code) { exit(ret_code); }

static int_t *fmem_ptr() {
  static int *res = 0;
  if (!res)
    res = malloc(STD_FMEM_SIZE * sizeof(int_t));
  return res;
}

int_t _std_fmemget(int_t pos) {
  std_check(pos >= 0, "std_fmemget: trying to access negative index");
  std_check(pos < STD_FMEM_SIZE,
            "std_fmemget: trying to access beyond fmem size");
  return fmem_ptr()[pos];
}

void _std_fmemset(int_t pos, int_t val) {
  std_check(pos >= 0, "std_fmemset: trying to access negative index");
  std_check(pos < STD_FMEM_SIZE,
            "std_fmemset: trying to access beyond fmem size");
  fmem_ptr()[pos] = val;
}

void _std_fmemcpy(int_t dst, int_t src, int_t n) {
  if (n <= 0)
    return;

  std_check(src >= 0, "std_fmemcpy: src negative index");
  std_check(src + n <= STD_FMEM_SIZE, "std_fmemcpy: src beyond fmem size");
  std_check(dst >= 0, "std_fmemcpy: dst negative index");
  std_check(dst + n <= STD_FMEM_SIZE, "std_fmemcpy: dst beyond fmem size");

  memmove(fmem_ptr() + dst, fmem_ptr() + src, n * sizeof(int_t));
}




// ====== ENTRY POINT ======

void _f1_main();


int main()
{
  _f1_main();
  return 0;
}
