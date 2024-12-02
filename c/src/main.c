#include <stdint.h>
#include <stdio.h>
#include "aoc.h"

int main(void) {
    vstr input = aoc_read(NULL);
    uint32_t result = day1_part2(&input);
    printf("result: %u\n", result);
    vstr_delete(&input);
    return 0;
}
