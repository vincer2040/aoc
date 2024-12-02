#include "base.h"
#include "vstr.h"
#include <stdio.h>

vstr aoc_read(const char* path) {
    vstr s = vstr_new();
    if (path == NULL) {
        char ch;
        while ((ch = fgetc(stdin)) != -1) {
            vstr_push_char(&s, ch);
        }
    } else {
        FILE* f = fopen(path, "r");
        aoc_assert(f != NULL);
        char ch;
        while ((ch = fgetc(f)) != -1) {
            vstr_push_char(&s, ch);
        }
        fclose(f);
    }
    return s;
}
