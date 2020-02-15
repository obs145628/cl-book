#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"

int_t queue_new() {
  int_t q = fm_alloc(3);
  std_fmemset(q, 0);
  std_fmemset(q + 1, 0);
  std_fmemset(q + 2, 0);
  return q;
}

void queue_free(int_t q) {
  int_t node = std_fmemget(q);
  while (node) {
    int_t next = std_fmemget(node + 1);
    fm_free(next);
    node = next;
  }

  fm_free(q);
}

int_t queue_size(int_t q) { return std_fmemget(q + 2); }

void queue_push(int_t q, int_t val) {
  int_t last = std_fmemget(q + 1);
  int_t new_last = fm_alloc(2);
  std_fmemset(new_last, val);
  std_fmemset(new_last + 1, 0);
  std_fmemset(q + 1, new_last);

  if (last == 0) {
    std_fmemset(q, new_last);
  } else {
    std_fmemset(last + 1, new_last);
  }

  std_fmemset(q + 2, std_fmemget(q + 2) + 1);
}

int_t queue_pop(int_t q) {
  int_t first = std_fmemget(q);
  panic_ifn(first);
  int_t res = std_fmemget(first);

  int_t new_first = std_fmemget(first + 1);
  std_fmemset(q, new_first);
  if (new_first == 0) {
    std_fmemset(q + 1, 0);
  }
  fm_free(first);

  std_fmemset(q + 2, std_fmemget(q + 2) - 1);
  return res;
}

void test1() {
  int_t q = queue_new();
  printnl_int(queue_size(q));

  queue_push(q, 10);
  queue_push(q, 18);
  queue_push(q, 23);
  queue_push(q, 45);

  while (queue_size(q)) {
    printnl_int(queue_pop(q));
  }

  queue_free(q);
}

void test2() {
  int_t q = queue_new();

  queue_push(q, 18);
  queue_push(q, 25);
  queue_push(q, 16);
  queue_push(q, 56);
  printnl_int(queue_pop(q));
  printnl_int(queue_pop(q));

  queue_push(q, 12);
  printnl_int(queue_pop(q));
  printnl_int(queue_pop(q));

  queue_push(q, 24);
  queue_push(q, 8);
  queue_push(q, -34);
  printnl_int(queue_pop(q));
  printnl_int(queue_pop(q));

  queue_free(q);
}

void test3() {
  int_t q = queue_new();
  int_t i = 0;
  while (i < 1000) {
    queue_push(q, 3 * i * i + 2 * i - 134);
    i += 1;
  }

  while (queue_size(q)) {
    printnl_int(queue_pop(q));
  }

  queue_free(q);
}

int main() {
  test1();
  test2();
  test3();
}
