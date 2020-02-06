#include <stdio.h>
int rank(int* arr, int len, int key) {
  if (len == 0)
    return -1;
  int beg = 0;
  int end = len - 1;

  while (beg <= end) {
    int mid = beg + (end - beg) / 2;
    int mid_val = arr[mid];
    if (key < mid_val)
      end = mid - 1;
    else if (key > mid_val)
      beg = mid + 1;
    else
      return mid;
  }

  return -1;
}

void test_empty() { printf("%d\n", (rank(NULL, 0, 56))); }

void test1() {
  int arr[1024];
  for (int i = 0; i < 10; ++i)
    arr[i] = i;
  

  printf("%d\n", rank(arr, 10, -50));
  for (int i = 0; i < 10; ++i)
    printf("%d\n", rank(arr, 10, i));
  printf("%d\n", rank(arr, 10, 63));
}

void test2() {
  int arr[1024];
  for (int i = 0; i < 10; ++i)
    arr[i] = i - 5;

  printf("%d\n", rank(arr, 10, -50));
  for (int i = -10; i < 10; ++i)
    printf("%d\n", rank(arr, 10, i));
  printf("%d\n", rank(arr, 10, 63));
}

void test3() {
  int arr[1024];
  for (int i = 0; i < 10; ++i)
    arr[i] = 2 * i;

  printf("%d\n", rank(arr, 10, -50));
  for (int i = 0; i < 20; ++i)
    printf("%d\n", rank(arr, 10, i));
  printf("%d\n", rank(arr, 10, 63));
}

int main() {
  test_empty();
  test1();
  test2();
  test3();
}
