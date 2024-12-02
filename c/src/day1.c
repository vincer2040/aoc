#include "line_iter.h"
#include "cwisstable.h"
#include "vstr.h"
#include <ctype.h>
#include <stdint.h>
#include <stdlib.h>

#define abs(a, b) (a) > (b) ? (a) - (b) : (b) - (a)

int cmp(const void* a, const void* b) {
    return *(const int*)(a) - *(const int*)(b);
}

uint32_t day1_part1(const vstr* input) {
    uint32_t result = 0;
    const char* data = vstr_data(input);
    uint64_t data_length = vstr_length(input);
    line_iter iter = line_iter_new(data, data_length);
    size_t count = num_lines(&iter);
    uint32_t* lhs = malloc(count * sizeof(uint32_t));
    uint32_t* rhs = malloc(count * sizeof(uint32_t));
    size_t ins = 0;
    while (iter.cur) {
        const char* cur = iter.cur;
        uint32_t lhs_cur = 0;
        uint32_t rhs_cur = 0;
        while (isdigit(*cur)) {
            lhs_cur = (lhs_cur * 10) + ((*cur) - 48);
            cur++;
        }
        while (*cur == ' ') {
            cur++;
        }
        while (isdigit(*cur)) {
            rhs_cur = (rhs_cur * 10) + ((*cur) - 48);
            cur++;
        }
        lhs[ins] = lhs_cur;
        rhs[ins] = rhs_cur;
        ins++;
        line_iter_next(&iter);
    }
    qsort(lhs, count, sizeof(uint32_t), cmp);
    qsort(rhs, count, sizeof(uint32_t), cmp);

    for (size_t i = 0; i < count; ++i) {
        result += abs(lhs[i], rhs[i]);
    }
    free(lhs);
    free(rhs);
    return result;
}

CWISS_DECLARE_FLAT_HASHMAP(u32_map, uint32_t, uint32_t);

uint32_t day1_part2(const vstr* input) {
    uint32_t result = 0;
    const char* data = vstr_data(input);
    uint64_t data_length = vstr_length(input);
    line_iter iter = line_iter_new(data, data_length);
    size_t count = num_lines(&iter);
    uint32_t* lhs = malloc(count * sizeof(uint32_t));
    uint32_t* rhs = malloc(count * sizeof(uint32_t));
    size_t ins = 0;
    while (iter.cur) {
        const char* cur = iter.cur;
        uint32_t lhs_cur = 0;
        uint32_t rhs_cur = 0;
        while (isdigit(*cur)) {
            lhs_cur = (lhs_cur * 10) + ((*cur) - 48);
            cur++;
        }
        while (*cur == ' ') {
            cur++;
        }
        while (isdigit(*cur)) {
            rhs_cur = (rhs_cur * 10) + ((*cur) - 48);
            cur++;
        }
        lhs[ins] = lhs_cur;
        rhs[ins] = rhs_cur;
        ins++;
        line_iter_next(&iter);
    }

    u32_map map = u32_map_new(30);
    for (size_t i = 0; i < count; ++i) {
        uint32_t cur = lhs[i];
        if (u32_map_contains(&map, &cur)) {
            u32_map_Iter iter = u32_map_find(&map, &cur);
            uint32_t x = u32_map_Iter_get(&iter)->val;
            result += (x * cur);
            continue;
        }
        uint32_t num_times = 0;
        for (size_t j = 0; j < count; ++j) {
            uint32_t rhs_cur = rhs[j];
            if (cur == rhs_cur) {
                num_times += 1;
            }
        }
        result += (num_times * cur);
        u32_map_Entry e = {cur, num_times};
        u32_map_insert(&map, &e);
    }

    u32_map_destroy(&map);

    free(lhs);
    free(rhs);
    return result;
}
