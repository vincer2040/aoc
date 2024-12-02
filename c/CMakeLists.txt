cmake_minimum_required(VERSION 3.10)

project(aoc VERSION 0.0.1)

set(CMAKE_C_COMPILER "gcc")

set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -std=c11 -Wall -fstack-clash-protection \
-fstack-protector-all -fstack-protector-strong -Werror=format-security -Werror=implicit-function-declaration -pipe -O2")

set(CMAKE_C_FLAGS_RELEASE "-std=c11 -Wall-fstack-clash-protection -fstack-protector-all \
-fstack-protector-strong -Werror=format-security -Werror=implicit-function-declaration -pipe -O2 -s -DNDEBUG")

add_executable(
    aoc
    src/main.c
    src/vstr.c
    src/reader.c
    src/line_iter.c
    src/day1.c
)
