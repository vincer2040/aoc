#ifndef __AOC_H__

#define __AOC_H__

#include "vstr.h"

vstr aoc_read(const char* path);
uint32_t day1_part1(const vstr *input);
uint32_t day1_part2(const vstr* input);

#endif /* __AOC_H__ */
