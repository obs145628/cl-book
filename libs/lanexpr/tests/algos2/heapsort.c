#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"
#include "../libs/rand.h"

int_t node_addr(int_t h, int_t k) { return h - 1 + k; }

void node_swap(int_t h, int_t p, int_t q) {
  int_t pval = std_fmemget(node_addr(h, p));
  std_fmemset(node_addr(h, p), std_fmemget(node_addr(h, q)));
  std_fmemset(node_addr(h, q), pval);
}

int_t node_cmp(int_t h, int_t p, int_t q) {
  return std_fmemget(node_addr(h, p)) - std_fmemget(node_addr(h, q));
}

void sink(int_t h, int_t k, int_t len) {
  int_t valid = 0;

  while (valid == 0) {
    if (2 * k > len) {
      valid = 1;
    } else {
      int_t j = 2 * k;
      if (j < len ? node_cmp(h, j + 1, j) > 0 : 0) {
        j = j + 1;
      }

      if (node_cmp(h, k, j) > 0) {
        valid = 1;
      } else {
        node_swap(h, k, j);
        k = j;
      }
    }
  }
}

void sort(int_t arr, int_t len) {

  int_t i = len / 2;
  while (i > 0) {
    sink(arr, i, len);
    i = i - 1;
  }

  i = len;
  while (i > 1) {
    node_swap(arr, 1, i);
    sink(arr, 1, i - 1);
    i = i - 1;
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
  int_t len = 100;
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
  int_t len = 207;
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
  int_t len = 178;
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
  int_t len = 675;
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
