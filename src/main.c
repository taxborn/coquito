#include <stddef.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

#include "file/reader.h"

void print_usage(char* executable) {
    // TODO: argument parsing options
    printf("[ERROR] Incorrect number of arguments passed.\n");
    printf("[USAGE] %s <file.cq>\n", executable);

    // Exit with an error.
    // TODO: Automatically go for main.cq? Seems unexpected but *could* be nice?
    exit(EXIT_FAILURE);
}

int main(int argc, char** argv) {
    // Check if the correct number of arguments were passed.
    // TODO: argument parsing
    if (argc != 2) print_usage(argv[0]);

    char* file_name = argv[1];
    char* contents = read_file(file_name);
    printf("[INFO] Starting compilation of '%s'.\n", file_name);

    return EXIT_SUCCESS;
}
