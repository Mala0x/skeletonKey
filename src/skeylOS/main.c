#include <stdio.h>

int main(void) {
  printf("Welcome to skeylos, the orchestrator behind the cool cyberdeck I am "
         "building!\n");

  printf(
      "Waddya wanna do? 1: Print the name, 2: Print another name, 3: Quit.\n");
  int buffer[1];
  scanf("%d", &buffer[0]);
  switch (buffer[0]) {
  case 1:
    printf("Skeylos\n");
    break;
  case 2:
    printf("Not skeylos\n");
    break;
  case 3:
    return -1;
    break;
  }
}
