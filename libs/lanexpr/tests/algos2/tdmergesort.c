#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"
#include "../libs/rand.h"

int_t cmp(int_t arr, int_t i, int_t j) {
  return std_fmemget(arr + i) - std_fmemget(arr + j);
}

void merge(int_t arr, int_t ws, int_t beg, int_t mid, int_t end) {
  int_t i = beg;
  int_t j = mid;
  std_fmemcpy(ws + beg, arr + beg, end - beg);

  int_t k = beg;
  while (k < end) {
    int_t read_i = 1;
    if (i == mid) {
      read_i = 0;
    } else if (j == end) {
      read_i = 1;
    } else {
      read_i = cmp(ws, i, j) < 0;
    }

    int_t val = 0;
    if (read_i) {
      val = std_fmemget(ws + i);
      i = i + 1;
    } else {
      val = std_fmemget(ws + j);
      j = j + 1;
    }

    std_fmemset(arr + k, val);
    k = k + 1;
  }
}

void sort_rec(int_t arr, int_t ws, int_t beg, int_t end) {
  if (end - beg > 1) {
    int_t mid = beg + (end - beg) / 2;
    sort_rec(arr, ws, beg, mid);
    sort_rec(arr, ws, mid, end);
    merge(arr, ws, beg, mid, end);
  }
}

void sort(int_t arr, int_t len) {
  int_t ws = fm_alloc(len);
  sort_rec(arr, ws, 0, len);
  fm_free(ws);
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
