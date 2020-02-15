#ifndef RAND_H_
#define RAND_H_

#include "alloc_v0.h"

int_t rng_new(int_t seed) {
  int_t rng = fm_alloc(1);
  std_fmemset(rng, seed);
  return rng;
}

void rng_free(int_t rng) { fm_free(rng); }

int_t rng_next(int_t rng) {
  int_t next = std_fmemget(rng);
  next = next * 1103515245 + 12345;
  std_fmemset(rng, next);
  return (next / 65536) % 32768;
}

#endif //! RAND_H_
