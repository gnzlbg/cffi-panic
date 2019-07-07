#include <stdio.h>
#include <setjmp.h>
#include <stdint.h>
#include "the_worst.cpp"

jmp_buf buf;

// Returns 0 on success, non-zero otherwise
extern "C" int32_t the_worst_wrapper(callback_t* callback) {
  printf("[the_worst_wrapper]: enter\n");
  try {
    the_worst(callback);
  } catch (const int32_t& i) {
    printf("[the_worst_wrapper]: exit error\n");
    return i;
  }
  printf("[the_worst_wrapper]: exit success\n");
  return 0;
}

// Can be called from the callback to return an error
extern "C" void callback_error(int32_t i) {
  printf("[callback_error]: enter with error %d\n", i);
  throw (int32_t)0;
}
