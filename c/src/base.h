#ifndef __BASE_H__

#define __BASE_H__

#include <stdio.h>
#include <stdlib.h>

#define aoc_assert(cond)                                                    \
    do {                                                                       \
        if (!(cond)) {                                                         \
            fprintf(stderr, "%s:%d ", __FILE__, __LINE__);                     \
            fprintf(stderr, "assertion failed: %s\n", #cond);                  \
            abort();                                                           \
        }                                                                      \
    } while (0);

#endif /* __BASE_H__ */
