#ifndef COQUITO_PARSER
#define COQUITO_PARSER

#include <stddef.h>

// -- LEXER --

typedef struct Lexer {
    const char* input;
    size_t position;
} Lexer;

// -- PARSER --

typedef struct Parser {
    Lexer lexer;
} Parser;

#endif
