#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

#include "reader.h"

char* read_file(char* path) {
    FILE *file;

    if ((file = fopen(path, "r")) == NULL) {
        free(file);
        printf("err = %d", errno);
        exit(EXIT_FAILURE);
    }

    // get the file size
    fseek(file, 0, SEEK_END);
    int file_size = ftell(file) - 1;
    rewind(file);

    printf("[INFO] End position of '%s' was %d.\n", path, file_size);

    char *buffer = (char *)malloc(file_size);
    
    // read the file into the buffer
    int ret = fread(buffer, sizeof(*buffer), file_size, file);
    fclose(file);
    if (ret != file_size) {
        printf("file read error: %d\n", ret);
        exit(EXIT_FAILURE);
    }

    // TODO: Check if UTF-8 file, and we can do that by checking the BOM (Byte order mark). For 
    // UTF-8, this is the codepoints (U+FEFF, 0xEFBBBF (?)), skip those if present.
    // This would involve checking the first 3 bytes for the U+FEFF codepoint. If tha is present, 
    //
    // if (starts_with_utf8_bom(path))
    //     ... parse UTF-8 codepoints

    buffer[file_size] = '\0';

    return buffer;
}
