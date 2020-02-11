#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"
#include "../libs/rand.h"

void heap_resize(int_t h, int_t new_cap) {
  int_t len = std_fmemget(h);
  int_t arr = std_fmemget(h + 2);
  int_t new_arr = fm_alloc(new_cap);

  std_fmemcpy(new_arr, arr, len);
  std_fmemset(h + 1, new_cap);
  std_fmemset(h + 2, new_arr);
  fm_free(arr);
}

int_t node_addr(int_t h, int_t k) {
  int_t arr = std_fmemget(h + 2);
  return arr - 1 + k;
}

void node_swap(int_t h, int_t p, int_t q) {
  int_t pval = std_fmemget(node_addr(h, p));
  std_fmemset(node_addr(h, p), std_fmemget(node_addr(h, q)));
  std_fmemset(node_addr(h, q), pval);
}

int_t node_cmp(int_t h, int_t p, int_t q) {
  return std_fmemget(node_addr(h, p)) - std_fmemget(node_addr(h, q));
}

void swim(int_t h, int_t k) {
  while (k > 1 ? node_cmp(h, k, k / 2) < 0 : 0) {
    node_swap(h, k, k / 2);
    k = k / 2;
  }
}

void sink(int_t h, int_t k) {
  int_t valid = 0;
  int_t len = std_fmemget(h);

  while (valid == 0) {
    if (2 * k > len) {
      valid = 1;
    } else {
      int_t j = 2 * k;
      if (j < len ? node_cmp(h, j + 1, j) < 0 : 0) {
        j = j + 1;
      }

      if (node_cmp(h, k, j) < 0) {
        valid = 1;
      } else {
        node_swap(h, k, j);
        k = j;
      }
    }
  }
}

int_t heap_new() {
  int_t h = fm_alloc(3);
  int_t arr = fm_alloc(4);
  std_fmemset(h, 0);
  std_fmemset(h + 1, 4);
  std_fmemset(h + 2, arr);
  return h;
}

void heap_free(int_t h) {
  fm_free(h + 2);
  fm_free(h);
}

void heap_push(int_t h, int_t val) {
  int_t len = std_fmemget(h);
  int_t cap = std_fmemget(h + 1);
  if (len == cap) {
    heap_resize(h, cap * 2);
  }

  std_fmemset(node_addr(h, len + 1), val);
  std_fmemset(h, len + 1);
  swim(h, len + 1);
}

int_t heap_pop(int_t h) {
  int_t len = std_fmemget(h);
  panic_ifn(len > 0);
  int_t res = std_fmemget(node_addr(h, 1));

  node_swap(h, 1, len);
  std_fmemset(h, len - 1);
  sink(h, 1);

  return res;
}

int_t heap_min(int_t h) {
  panic_ifn(heap_size(h) > 0);
  return std_fmemget(node_addr(h, 1));
}

int_t heap_size(int_t h) { return std_fmemget(h); }

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

void sort(int_t arr, int_t len) {
  int_t h = heap_new();

  int_t i = 0;
  while (i < len) {
    heap_push(h, std_fmemget(arr + i));
    i = i + 1;
  }

  i = 0;
  while (i < len) {
    std_fmemset(arr + i, heap_pop(h));
    i = i + 1;
  }

  heap_free(h);
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
