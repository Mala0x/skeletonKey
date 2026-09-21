#include <stdbool.h>
#include <stdio.h>
#include <string.h>

typedef struct {
  bool verbose;
  bool log_to_file;
} program_flags;

void parse_arguments(int argc, char **argv, program_flags *program_flags) {
  for (int i = 0; i < argc; i++) {
    if (strcmp(argv[i], "--v") == 0) {
      program_flags->verbose = true;
      printf("Verbose mode is set to true!\n");
    } else if (strcmp(argv[i], "--log") == 0) {
      program_flags->log_to_file = true;
      printf("Log to file mode is set to true!\n");
    }
  }
}

int main(int argc, char **argv) {

  printf("Hello, world!\n");

  program_flags curr_prog_flags = {false, false};

  parse_arguments(argc, argv, &curr_prog_flags);

  return 0;
}
