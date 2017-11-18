#include "myclib.h"
#include <string.h>

uint64_t count_byte_doubles(char * str) {
  uint64_t count = 0;
  int len = strlen(str);
  while (len--) {
    if (str[0] == str[1]) count++;
    str++;
  }
  return count;
}
