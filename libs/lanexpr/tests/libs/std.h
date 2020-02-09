#ifndef LESTD_H_
#define LESTD_H_

#include <stddef.h>
#include <stdint.h>

int getchar();
int putchar(int);
void exit(int);

void *memmove(void *dst, const void *src, size_t n);

void *malloc(size_t);

typedef int32_t int_t;

#define STD_FMEM_SIZE (16 * 1024 * 1024)

void std_check(int val, const char *mess) {
  if (val)
    return;
  for (; *mess; ++mess)
    putchar(*mess);
  putchar('\n');
  exit(26);
}

void std_putc(int_t byte_val) { putchar(byte_val); }

int_t std_getc() { return getchar(); }

void std_exit(int_t ret_code) { exit(ret_code); }

int_t *fmem_ptr() {
  static int *res = 0;
  if (!res)
    res = malloc(STD_FMEM_SIZE * sizeof(int_t));
  return res;
}

int_t std_fmemget(int_t pos) {
  std_check(pos >= 0, "std_fmemget: trying to access negative index");
  std_check(pos < STD_FMEM_SIZE,
            "std_fmemget: trying to access beyond fmem size");
  return fmem_ptr()[pos];
}

// Write the flat memory 32b entry at index pos
void std_fmemset(int_t pos, int_t val) {
  std_check(pos >= 0, "std_fmemset: trying to access negative index");
  std_check(pos < STD_FMEM_SIZE,
            "std_fmemset: trying to access beyond fmem size");
  fmem_ptr()[pos] = val;
}

// Copy n entries starting at index src, to the n entries starting at index dst
// src and dst can overlap
void std_fmemcpy(int_t dst, int_t src, int_t n) {
  if (n <= 0)
    return;

  std_check(src >= 0, "std_fmemcpy: src negative index");
  std_check(src + n <= STD_FMEM_SIZE, "std_fmemcpy: src beyond fmem size");
  std_check(dst >= 0, "std_fmemcpy: dst negative index");
  std_check(dst + n <= STD_FMEM_SIZE, "std_fmemcpy: dst beyond fmem size");

  memmove(fmem_ptr() + dst, fmem_ptr() + src, n * sizeof(int_t));
}

#define STD_FMEM_SIZE (16 * 1024 * 1024)

#endif //! LESTD_H_
