#ifndef __LINE_ITER_H__

#define __LINE_ITER_H__

#include <stdint.h>

#include <stddef.h>

typedef struct {
    size_t pos;
    size_t len;
    size_t cur_len;
    size_t next_len;
    const char* input;
    char* cur;
    char* next;
} line_iter;

line_iter line_iter_new(const char* input, size_t len);
size_t num_lines(line_iter* iter);
void line_iter_next(line_iter* iter);

#endif /* __LINE_ITER_H__ */
