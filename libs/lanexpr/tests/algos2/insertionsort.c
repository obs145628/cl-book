#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"
#include "../libs/rand.h"

int_t cmp(int_t arr, int_t i, int_t j) {
  return std_fmemget(arr + i) - std_fmemget(arr + j);
}

void swap(int_t arr, int_t i, int_t j) {
  int_t vi = std_fmemget(arr + i);
  std_fmemset(arr + i, std_fmemget(arr + j));
  std_fmemset(arr + j, vi);
}

void sort(int_t arr, int_t len) {
  int_t i = 0;
  while (i < len) {

    int_t j = i;
    while (j > 0 && cmp(arr, j, j - 1) < 0) {
      swap(arr, j, j - 1);
      j = j - 1;
    }

    i = i + 1;
  }
}

void print_arr(int_t arr, int_t len) {
  std_putc(91);

  int_t i = 0;
  while (i < len) {
    print_int(std_fmemget(arr + i));
    if (i + 1 < len) {
      std_putc(44);
      std_putc(32);
    }
    i = i + 1;
  }

  std_putc(93);
  std_putc(10);
}

void test1() {
  int_t arr = fm_alloc(7);
  std_fmemset(arr + 0, 12);
  std_fmemset(arr + 1, 8);
  std_fmemset(arr + 2, -6);
  std_fmemset(arr + 3, 25);
  std_fmemset(arr + 4, 18);
  std_fmemset(arr + 5, 12);
  std_fmemset(arr + 6, -2);
  sort(arr, 7);
  print_arr(arr, 7);
  fm_free(arr);
}

void test2() {
  int_t len = 50;
  int_t arr = fm_alloc(len);

  int_t i = 0;
  while (i < len) {
    std_fmemset(arr + i, -2 * i * i + 5 * i - 8);
    i += 1;
  }

  sort(arr, len);
  print_arr(arr, len);
  fm_free(arr);
}

void test3() {
  int_t len = 107;
  int_t arr = fm_alloc(len);

  int_t i = 0;
  while (i < len) {
    std_fmemset(arr + i, 1000 + 12 * i);
    i += 1;
  }

  sort(arr, len);
  print_arr(arr, len);
  fm_free(arr);
}

void test4() {
  int_t len = 78;
  int_t arr = fm_alloc(len);

  int_t i = 0;
  while (i < len) {
    std_fmemset(arr + i, 1000 - 12 * i);
    i += 1;
  }

  sort(arr, len);
  print_arr(arr, len);
  fm_free(arr);
}

void test5() {
  int_t len = 113;
  int_t arr = fm_alloc(len);
  int_t rng = rng_new(78);

  int_t i = 0;
  while (i < len) {
    std_fmemset(arr + i, rng_next(rng));
    i = i + 1;
  }

  sort(arr, len);
  print_arr(arr, len);
  rng_free(rng);
  fm_free(arr);
}

int main() {
  test1();
  test2();
  test3();
  test4();
  test5();
}
