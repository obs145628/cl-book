#include "../libs/alloc_v0.h"
#include "../libs/debug.h"
#include "../libs/print.h"
#include "../libs/rand.h"

int_t ivec_new(int_t size) {
  int_t v = fm_alloc(size + 1);
  std_fmemset(v, size);
  return v;
}

int_t ivec_new_fill(int_t size, int_t init)
{
  int_t v = ivec_new(size);
  int_t i = 0;
  while (i < size) {
    std_fmemset(v + 1 + i, init);
    i = i + 1;
  }
  return v;
}

int_t ivec_new_rand(int_t size, int_t rng) {
  int_t v = ivec_new(size);
  int_t i = 0;
  while (i < size) {
    std_fmemset(v + i + 1, rng_next(rng) % 100);
    i = i + 1;
  }
  return v;
}

void ivec_free(int_t v) {
  fm_free(v);
}


void ivec_print(int_t v) {
  int_t len = std_fmemget(v);
  std_putc(91);

  int_t i = 0;
  while (i < len) {
    print_int(std_fmemget(v + 1 + i));
    if (i + 1 < len) {
      std_putc(44);
      std_putc(32);
    }
    i = i + 1;
  }

  std_putc(93);
  std_putc(10);
}

int_t ivec_size(int_t v) {
  return std_fmemget(v);
}

int_t ivec_iget(int_t v, int_t pos) {
  int_t len = std_fmemget(v);
  panic_ifn(pos < len);
  return std_fmemget(v + 1 + pos);
}

void ivec_iset(int_t v, int_t pos, int_t val) {
  int_t len = std_fmemget(v);
  panic_ifn(pos < len);
  std_fmemset(v + 1 + pos, val);
}


int_t iadd_vv(int_t u, int_t v) {
  int_t len = ivec_size(u);
  panic_ifn(len == ivec_size(v));
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, ivec_iget(u, i) + ivec_iget(v, i));
    i = i + 1;
  }
  return res;
}

int_t isub_vv(int_t u, int_t v) {
  int_t len = ivec_size(u);
  panic_ifn(len == ivec_size(v));
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, ivec_iget(u, i) - ivec_iget(v, i));
    i = i + 1;
  }
  return res;
}

int_t imul_vv(int_t u, int_t v) {
  int_t len = ivec_size(u);
  panic_ifn(len == ivec_size(v));
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, ivec_iget(u, i) * ivec_iget(v, i));
    i = i + 1;
  }
  return res;
}

int_t idiv_vv(int_t u, int_t v) {
  int_t len = ivec_size(u);
  panic_ifn(len == ivec_size(v));
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, ivec_iget(u, i) / ivec_iget(v, i));
    i = i + 1;
  }
  return res;
}

int_t iadd_vs(int_t u, int_t x) {
  int_t len = ivec_size(u);
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, ivec_iget(u, i) + x);
    i = i + 1;
  }
  return res;
}

int_t isub_vs(int_t u, int_t x) {
  int_t len = ivec_size(u);
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, ivec_iget(u, i) - x);
    i = i + 1;
  }
  return res;
}

int_t imul_vs(int_t u, int_t x) {
  int_t len = ivec_size(u);
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, ivec_iget(u, i) * x);
    i = i + 1;
  }
  return res;
}

int_t idiv_vs(int_t u, int_t x) {
  int_t len = ivec_size(u);
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, ivec_iget(u, i) / x);
    i = i + 1;
  }
  return res;
}

int_t isub_sv(int_t x, int_t u) {
  int_t len = ivec_size(u);
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, x - ivec_iget(u, i));
    i = i + 1;
  }
  return res;
}

int_t idiv_sv(int_t x, int_t u) {
  int_t len = ivec_size(u);
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, x / ivec_iget(u, i));
    i = i + 1;
  }
  return res;
}

int_t ineg_v(int_t u) {
  return isub_sv(0, u);
}

int_t isqrt_s(int_t n) {
  int_t x = n;
  int_t y = 1;
  while (x > y) {
    x = (x + y) / 2;
    y = n / x;
  }
  return x;
}

int_t isqrt_v(int_t u) {
  int_t len = ivec_size(u);
  int_t res = ivec_new(len);

  int_t i = 0;
  while (i < len) {
    ivec_iset(res, i, isqrt_s(ivec_iget(u, i)));
    i = i + 1;
  }
  return res;
}

int_t idot_vv(int_t u, int_t v) {
  int_t len = ivec_size(u);
  int_t res = 0;
  panic_ifn(len == ivec_size(v));

  int_t i = 0;
  while (i < len) {
    res += ivec_iget(u, i) * ivec_iget(v, i);
    i = i + 1;
  }

  return res;
}

int_t  idistsq_vv(int_t u, int_t v) {
  int_t sub = isub_vv(u, v);
  int_t res = idot_vv(sub, sub);
  ivec_free(sub);
  return res;
}

int_t idist_vv(int_t u, int_t v) {
  return isqrt_s(idistsq_vv(u, v));
}


void test1() {
  int_t v = ivec_new_fill(6, 12);
  ivec_print(v);
  printnl_int(ivec_iget(v, 3));

  int_t i = 1;
  while (i < ivec_size(v)) {
    ivec_iset(v, i, 4*i*i - 10);
    i = i + 1;
  }

  i = i / (i + 1);
  while (i < ivec_size(v)) {
    printnl_int(ivec_iget(v, i));
    i = i + 1;
  }

  ivec_print(v);
  ivec_free(v);
}

void test2() {
  int_t rng = rng_new(47);
  int_t v1 = ivec_new_rand(8, rng);
  int_t v2 = ivec_new_rand(5, rng);
  ivec_print(v1);
  ivec_print(v2);
  ivec_free(v1);
  ivec_free(v2);
  rng_free(rng);
}

void test3() {
  int_t rng = rng_new(47);
  int_t v1 = ivec_new_rand(8, rng);
  int_t v2 = ivec_new_rand(8, rng);
  ivec_print(v1);
  ivec_print(v2);

  int_t v3 = iadd_vv(v1, v2);
  ivec_print(v3);

  int_t v4 = isub_vv(v1, v2);
  ivec_print(v4);

  int_t v5 = imul_vv(v1, v2);
  ivec_print(v5);

  int_t v6 = idiv_vv(v1, v2);
  ivec_print(v6);

  int_t v7 = ineg_v(v3);
  ivec_print(v7);

  ivec_free(v1);
  ivec_free(v2);
  ivec_free(v3);
  ivec_free(v4);
  ivec_free(v5);
  ivec_free(v6);
  ivec_free(v7);
  rng_free(rng);
}

void test4() {
  int_t rng = rng_new(3465);
  int_t v1 = ivec_new_rand(8, rng);
  ivec_print(v1);

  int_t v2 = iadd_vs(v1, 12);
  ivec_print(v2);

  int_t v3 = isub_vs(v1, 7);
  ivec_print(v3);

  int_t v4 = imul_vs(v1, 3);
  ivec_print(v4);

  int_t v5 = idiv_vs(v1, 2);
  ivec_print(v5);

  int_t v6 = isub_sv(100, v1);
  ivec_print(v6);

  int_t v7 = idiv_sv(1001, v1);
  ivec_print(v7);

  ivec_free(v1);
  ivec_free(v2);
  ivec_free(v3);
  ivec_free(v4);
  ivec_free(v5);
  ivec_free(v6);
  ivec_free(v7);
  rng_free(rng);
}

void test5() {
  int_t rng = rng_new(712);
  int_t v1 = ivec_new_rand(8, rng);
  int_t v2 = imul_vv(v1, v1);
  ivec_print(v1);
  ivec_print(v2);

  int_t v3 = isqrt_v(v2);
  ivec_print(v3);

  int_t v4 = ivec_new_rand(8, rng);
  ivec_print(v4);
  
  printnl_int(idot_vv(v1, v4));
  printnl_int(idist_vv(v1, v4));
  printnl_int(idistsq_vv(v1, v1));

  ivec_free(v1);
  ivec_free(v2);
  ivec_free(v3);
  ivec_free(v4);
  rng_free(rng);
}


int main()
{
  test1();
  test2();
  test3();
  test4();
  test5();
}
