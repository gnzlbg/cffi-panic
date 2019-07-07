#include <stdio.h>
typedef void(callback_t)();

void the_worst(callback_t* callback) {
  printf("[the_worst]: enter\n");
  callback();
  printf("[the_worst]: exit\n");
}
