#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"

int_t id_addr(int_t uf, int_t idx) { return uf + 1 + 2 * idx; }

int_t size_addr(int_t uf, int_t idx) { return uf + 1 + 2 * idx + 1; }

int_t uf_new(int_t n) {
  int_t uf = fm_alloc(1 + 2 * n);
  std_fmemset(uf, n);
  int_t i = 0;
  while (i < n) {
    std_fmemset(id_addr(uf, i), i);
    std_fmemset(size_addr(uf, i), 1);
    i = i + 1;
  }

  return uf;
}

void uf_free(int_t uf) { fm_free(uf); }

void uf_union(int_t uf, int_t p, int_t q) {
  int_t pr = uf_find(uf, p);
  int_t qr = uf_find(uf, q);

  if (pr != qr) {
    int_t pr_size = std_fmemget(size_addr(uf, pr));
    int_t qr_size = std_fmemget(size_addr(uf, qr));
    std_fmemset(uf, std_fmemget(uf) - 1);

    if (pr_size < qr_size) {
      std_fmemset(id_addr(uf, pr), qr);
      std_fmemset(size_addr(uf, qr), qr_size + pr_size);
    } else {
      std_fmemset(id_addr(uf, qr), pr);
      std_fmemset(size_addr(uf, pr), pr_size + qr_size);
    }
  }
}

int_t uf_find(int_t uf, int_t p) {
  while (p != std_fmemget(id_addr(uf, p))) {
    p = std_fmemget(id_addr(uf, p));
  }
  return p;
}

int_t uf_connected(int_t uf, int_t p, int_t q) {
  return uf_find(uf, p) == uf_find(uf, q);
}

int_t uf_count(int_t uf) { return std_fmemget(uf); }

void test1() {
  int_t g = uf_new(3);
  printnl_int(uf_find(g, 0));
  printnl_int(uf_find(g, 1));
  printnl_int(uf_find(g, 2));
  printnl_int(uf_connected(g, 0, 1));
  printnl_int(uf_connected(g, 0, 2));
  printnl_int(uf_connected(g, 1, 2));
  printnl_int(uf_count(g));
  uf_free(g);
}

void test2() {
  int_t g = uf_new(3);
  uf_union(g, 0, 1);
  printnl_int(uf_find(g, 0));
  printnl_int(uf_find(g, 1));
  printnl_int(uf_find(g, 2));
  printnl_int(uf_connected(g, 0, 1));
  printnl_int(uf_connected(g, 0, 2));
  printnl_int(uf_connected(g, 1, 2));
  printnl_int(uf_count(g));
  uf_free(g);
}

void test3() {
  int_t g = uf_new(3);
  uf_union(g, 0, 1);
  uf_union(g, 1, 2);
  printnl_int(uf_find(g, 0));
  printnl_int(uf_find(g, 1));
  printnl_int(uf_find(g, 2));
  printnl_int(uf_connected(g, 0, 1));
  printnl_int(uf_connected(g, 0, 2));
  printnl_int(uf_connected(g, 1, 2));
  printnl_int(uf_count(g));
  uf_free(g);
}

void test4() {
  int_t g = uf_new(12);
  uf_union(g, 0, 2);
  uf_union(g, 10, 5);
  uf_union(g, 4, 2);
  uf_union(g, 8, 9);
  uf_union(g, 0, 7);
  uf_union(g, 1, 7);
  uf_union(g, 3, 11);
  uf_union(g, 5, 8);
  uf_union(g, 4, 7);

  int_t i = 0;
  while (i < 12) {
    printnl_int(uf_find(g, i));
    i += 1;
  }
  printnl_int(uf_count(g));
  uf_free(g);
}

int main() {
  test1();
  test2();
  test3();
  test4();
}
