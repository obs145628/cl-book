#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"
#include "../libs/rand.h"

int_t node_new(int_t key, int_t val, int_t left, int_t right, int_t parent) {
  int_t node = fm_alloc(5);
  std_fmemset(node, key);
  std_fmemset(node + 1, val);
  std_fmemset(node + 2, left);
  std_fmemset(node + 3, right);
  std_fmemset(node + 4, parent);
  return node;
}

void node_free(int_t node) {
  if (node) {
    node_free(std_fmemget(node + 2));
    node_free(std_fmemget(node + 3));
    fm_free(node);
  }
}

void node_swap(int_t a, int_t b) {
  int_t a_key = std_fmemget(a);
  int_t a_val = std_fmemget(a + 1);
  std_fmemset(a, std_fmemget(b));
  std_fmemset(a + 1, std_fmemget(b + 1));
  std_fmemset(b, a_key);
  std_fmemset(b + 1, a_val);
}

int_t node_min(int_t node) {
  int_t left = std_fmemget(node + 2);
  return left == 0 ? node : node_min(left);
}

int_t node_find(int_t node, int_t key) {
  if (node == 0) {
    return 0;
  } else {
    int_t cmp = key - std_fmemget(node);
    if (cmp < 0)
      return node_find(std_fmemget(node + 2), key);
    else if (cmp > 0)
      return node_find(std_fmemget(node + 3), key);
    else
      return node;
  }
}

int_t node_put(int_t node_ptr, int_t parent, int_t key, int_t val) {
  int_t node = std_fmemget(node_ptr);
  if (node == 0) {
    std_fmemset(node_ptr, node_new(key, val, 0, 0, parent));
    return 1;
  } else {
    int_t cmp = key - std_fmemget(node);
    if (cmp == 0) {
      std_fmemset(node + 1, val);
      return 0;
    } else {
      int_t child_ptr = cmp < 0 ? node + 2 : node + 3;
      return node_put(child_ptr, node, key, val);
    }
  }
}

int_t node_del(int_t node_ptr, int_t key) {
  int_t node = std_fmemget(node_ptr);
  if (node == 0) {
    return 0;
  }

  else {
    int_t cmp = key - std_fmemget(node);
    if (cmp) {
      int_t child_ptr = cmp < 0 ? node + 2 : node + 3;
      return node_del(child_ptr, key);
    } else {
      int_t left = std_fmemget(node + 2);
      int_t right = std_fmemget(node + 3);

      if (left == 0) {
        std_fmemset(node_ptr, right);
        fm_free(node);
      } else if (right == 0) {
        std_fmemset(node_ptr, left);
        fm_free(node);
      } else {
        int_t rep_node = node_min(right);
        node_swap(node, rep_node);
        panic_ifn(node_del(node + 3, key));
      }

      return 1;
    }
  }
}

int_t node_next(int_t node) {
  int_t right = std_fmemget(node + 3);
  if (right) {
    return node_min(right);
  } else {
    int_t parent = std_fmemget(node + 4);
    if (parent == 0) {
      return 0;
    } else {
      int_t from_left = std_fmemget(parent + 2) == node;
      return from_left ? parent : node_next(parent);
    }
  }
}

int_t table_new() {
  int_t st = fm_alloc(2);
  std_fmemset(st, 0);
  std_fmemset(st + 1, 0);
  return st;
}

void table_free(int_t st) {
  node_free(std_fmemget(st));
  fm_free(st);
}

int_t table_put(int_t st, int_t key, int_t val) {
  int_t res = node_put(st, 0, key, val);
  std_fmemset(st + 1, std_fmemget(st + 1) + res);
  return res;
}

int_t table_delete(int_t st, int_t key) {
  int_t res = node_del(st, key);
  std_fmemset(st + 1, std_fmemget(st + 1) - res);
  return res;
}

int_t table_get(int_t st, int_t key) {
  int_t node = node_find(std_fmemget(st), key);
  panic_ifn(node);
  return std_fmemget(node + 1);
}

int_t table_contains(int_t st, int_t key) {
  return (node_find(std_fmemget(st), key) == 0) == 0;
}

int_t table_size(int_t st) { return std_fmemget(st + 1); }

int_t table_it_new(int_t st) {
  int_t root = std_fmemget(st);
  int_t node = root ? node_min(root) : 0;

  int_t it = fm_alloc(1);
  std_fmemset(it, node);
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
  if (node) {
    node = node_next(node);
  }
  std_fmemset(it, node);
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
