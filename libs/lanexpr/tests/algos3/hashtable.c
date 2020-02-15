#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"
#include "../libs/rand.h"

int_t node_new(int_t key, int_t val, int_t head) {
  int_t res = fm_alloc(3);
  std_fmemset(res, key);
  std_fmemset(res + 1, val);
  std_fmemset(res + 2, head);
  return res;
}

void ll_free(int_t l) {
  while (l) {
    int_t next = std_fmemget(l + 2);
    fm_free(l);
    l = next;
  }
}

int_t ll_find(int_t l, int_t key) {
  int_t target = 0;

  while (l) {
    if (std_fmemget(l) == key) {
      target = l;
      l = 0;
    } else {
      l = std_fmemget(l + 2);
    }
  }

  return target;
}

int_t ll_find_ptr(int_t l_ptr, int_t key) {
  int_t target = 0;
  int_t l = std_fmemget(l_ptr);

  while (l) {
    if (std_fmemget(l) == key) {
      target = l_ptr;
      l = 0;
    } else {
      l_ptr = l + 2;
      l = std_fmemget(l_ptr);
    }
  }

  return target;
}

int_t hash_fn(int_t x) { return x * 2654435761; }

int_t hash_key(int_t x, int_t len) {
  x = hash_fn(x);
  x = x > 0 ? x : -x;
  return x % len;
}

int_t table_new() {
  int_t n = 37;
  int_t st = fm_alloc(2 + n);
  std_fmemset(st, n);
  std_fmemset(st + 1, 0);

  int_t i = 0;
  while (i < n) {
    std_fmemset(st + 2 + i, 0);
    i = i + 1;
  }

  return st;
}

void table_free(int_t st) {
  int_t n = std_fmemget(st);
  int_t i = 0;
  while (i < n) {
    ll_free(std_fmemget(st + 2 + i));
    i = i + 1;
  }

  fm_free(st);
}

int_t table_put(int_t st, int_t key, int_t val) {
  int_t n = std_fmemget(st);
  int_t idx = hash_key(key, n);
  int_t head = std_fmemget(st + 2 + idx);
  int_t node = ll_find(head, key);

  if (node) {
    std_fmemset(node + 1, val);
    return 0;
  } else {
    head = node_new(key, val, head);
    std_fmemset(st + 2 + idx, head);
    std_fmemset(st + 1, std_fmemget(st + 1) + 1);
    return 1;
  }
}

int_t table_delete(int_t st, int_t key) {
  int_t n = std_fmemget(st);
  int_t idx = hash_key(key, n);
  int_t head_ptr = st + 2 + idx;
  int_t node_ptr = ll_find_ptr(head_ptr, key);

  if (node_ptr) {
    int_t node = std_fmemget(node_ptr);
    panic_ifn(node);
    std_fmemset(node_ptr, std_fmemget(node + 2));
    fm_free(node);
    std_fmemset(st + 1, std_fmemget(st + 1) - 1);
    return 1;
  } else {
    return 0;
  }
}

int_t table_get(int_t st, int_t key) {
  int_t n = std_fmemget(st);
  int_t idx = hash_key(key, n);
  int_t head = std_fmemget(st + 2 + idx);
  int_t node = ll_find(head, key);
  panic_ifn(node);
  return std_fmemget(node + 1);
}

int_t table_contains(int_t st, int_t key) {
  int_t n = std_fmemget(st);
  int_t idx = hash_key(key, n);
  int_t head = std_fmemget(st + 2 + idx);
  return (ll_find(head, key) == 0) == 0;
}

int_t table_size(int_t st) { return std_fmemget(st + 1); }

void table_it_next(int_t it);
int_t table_it_new(int_t st) {
  int_t it = fm_alloc(3);
  std_fmemset(it, 0);
  std_fmemset(it + 1, -1);
  std_fmemset(it + 2, st);
  table_it_next(it);
  return it;
}

void table_it_free(int_t it) { fm_free(it); }

int_t table_it_is_end(int_t it) {
  int_t node = std_fmemget(it);
  return node == 0;
}

int_t table_it_get_key(int_t it) {
  int_t node = std_fmemget(it);
  panic_ifn(node);
  return std_fmemget(node);
}

int_t table_it_get_val(int_t it) {
  int_t node = std_fmemget(it);
  panic_ifn(node);
  return std_fmemget(node + 1);
}

void table_it_next(int_t it) {
  int_t node = std_fmemget(it);
  int_t idx = std_fmemget(it + 1);
  int_t st = std_fmemget(it + 2);
  int_t n = std_fmemget(st);

  if (idx < n) {
    if (node)
      node = std_fmemget(node + 2);
    if (!node) {
      int_t found = 0;
      idx = idx + 1;
      while (found == 0) {
        if (idx == n) {
          found = 1;
        } else if (std_fmemget(st + 2 + idx)) {
          found = 1;
        } else {
          idx = idx + 1;
        }
      }

      node = idx < n ? std_fmemget(st + 2 + idx) : 0;
      std_fmemset(it + 1, idx);
    }

    std_fmemset(it, node);
  }
}


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
  sort(arr, len);
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

void sort2(int_t arr1, int arr2, int_t len) {
  int_t i = 0;
  while (i < len) {

    int_t j = i;
    while (j > 0 && cmp(arr1, j, j - 1) < 0) {
      swap(arr1, j, j - 1);
      swap(arr2, j, j - 1);
      j = j - 1;
    }

    i = i + 1;
  }
}
void print_arr2(int_t arr1, int arr2, int_t len) {
  sort2(arr1, arr2, len);
  std_putc(91);

  int_t i = 0;
  while (i < len) {
    std_putc(40);
    print_int(std_fmemget(arr1 + i));
    std_putc(59);
    print_int(std_fmemget(arr2 + i));
    std_putc(41);
    if (i + 1 < len) {
      std_putc(44);
      std_putc(32);
    }
    i = i + 1;
  }

  std_putc(93);
  std_putc(10);
}

void print_keys(int_t st) {
  int_t len = table_size(st);
  int_t keys = fm_alloc(len);
  int_t it = table_it_new(st);
  int_t i = 0;

  while (i < len) {
    std_fmemset(keys + i, table_it_get_key(it));
    table_it_next(it);
    i = i + 1;
  }

  print_arr(keys, len);
  table_it_free(it);
  fm_free(keys);
}

void print_vals(int_t st) {
  int_t len = table_size(st);
  int_t vals = fm_alloc(len);
  int_t it = table_it_new(st);
  int_t i = 0;

  while (i < len) {
    std_fmemset(vals + i, table_it_get_val(it));
    table_it_next(it);
    i = i + 1;
  }

  print_arr(vals, len);
  table_it_free(it);
  fm_free(vals);
}

void print_table(int_t st) {
  int_t len = table_size(st);
  int_t keys = fm_alloc(len);
  int_t vals = fm_alloc(len);
  int_t it = table_it_new(st);
  int_t i = 0;

  while (i < len) {
    std_fmemset(keys + i, table_it_get_key(it));
    std_fmemset(vals + i, table_it_get_val(it));
    table_it_next(it);
    i = i + 1;
  }

  print_arr2(keys, vals, len);
  table_it_free(it);
  fm_free(keys);
  fm_free(vals);
}

void test1() {
  int_t st = table_new();
  print_keys(st);
  print_vals(st);
  print_table(st);
  table_free(st);
}

void test2() {
  int_t st = table_new();
  printnl_int(table_put(st, 3, 78));
  printnl_int(table_put(st, 6, 4));
  printnl_int(table_put(st, 2, 45));
  printnl_int(table_put(st, 1, 27));
  printnl_int(table_put(st, 2, 37));
  printnl_int(table_put(st, 8, 44));

  int_t i = 0;
  while (i < 10) {
    printnl_int(table_contains(st, i));
    i = i + 1;
  }

  print_keys(st);
  print_vals(st);
  print_table(st);
  table_free(st);
}

void test3() {
  int_t st = table_new();
  int_t i = 0;
  while (i < 20) {
    printnl_int(table_put(st, i, i * i));
    i = i + 1;
  }

  i = 0;
  while (i < 20) {
    printnl_int(table_contains(st, i));
    i = i + 1;
  }
  print_table(st);

  i = 0;
  while (i < 20) {
    printnl_int(table_delete(st, i));
    i = i + 1;
  }

  print_table(st);
  table_free(st);
}

void test4() {
  int_t st = table_new();
  int_t i = -40;
  while (i < 40) {
    printnl_int(table_put(st, i, i * i));
    i = i + 1;
  }

  i = -40;
  while (i < 40) {
    printnl_int(table_contains(st, i));
    i = i + 1;
  }
  print_table(st);

  i = -12;
  while (i < 4) {
    printnl_int(table_delete(st, i));
    i = i + 1;
  }

  i = -40;
  while (i < 40) {
    printnl_int(table_contains(st, i));
    i = i + 1;
  }
  print_table(st);

  i = 4;
  while (i < 28) {
    printnl_int(table_put(st, i, 4 * i * i - 5));
    i = i + 1;
  }

  i = -40;
  while (i < 40) {
    printnl_int(table_contains(st, i));
    i = i + 1;
  }
  print_table(st);

  i = -37;
  while (i < 8) {
    printnl_int(table_delete(st, i));
    i = i + 1;
  }

  i = -40;
  while (i < 40) {
    printnl_int(table_contains(st, i));
    i = i + 1;
  }
  print_table(st);

  i = 16;
  while (i < 39) {
    printnl_int(table_put(st, i, -2 * i + 50));
    i = i + 1;
  }

  i = -40;
  while (i < 40) {
    printnl_int(table_contains(st, i));
    i = i + 1;
  }
  print_table(st);

  table_free(st);
}

int main() {
  test1();
  test2();
  test3();
  test4();
}
