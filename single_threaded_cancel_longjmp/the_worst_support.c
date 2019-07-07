#include <stdio.h>
#include <setjmp.h>
#include <stdint.h>
#include "the_worst.c"

jmp_buf buf;

// Returns 0 on success, non-zero otherwise
int32_t the_worst_wrapper(callback_t* callback) {
  printf("[the_worst_wrapper]: enter\n");
  switch (setjmp(buf)) {
      case 0: the_worst(callback); break;
      case 42: printf("[the_worst_wrapper]: exit error\n"); return 42;
  }

  printf("[the_worst_wrapper]: exit success\n");
  return 0;
}

// Can be called from the callback to return an error
void callback_error(int32_t i) {
  printf("[callback_error]: enter with error %d\n", i);
  longjmp(buf, i);
}
