#include <stdio.h>
#include <stdint.h>
#include "the_worst.cpp"

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

extern "C" uint32_t my_callback();

// Can be called from the callback to return an error
extern "C" void my_callback_wrapper() {
  int error = my_callback();
  if (error != 0) {
      printf("[callback_error]: enter with error %d\n", error);
      throw (int32_t)0;
  }
  return;
}
