#ifndef __VSTR_H__

#define __VSTR_H__

#include <stdint.h>

#define VSTR_SMALL_MAX_LEN 23
#define VSTR_LARGE_MAX_LEN ((((uint64_t)(1)) << 56) - 1)

#ifndef vstr_alloc
#include <stdlib.h>
#define vstr_malloc malloc
#define vstr_realloc realloc
#define vstr_free free
#endif

typedef struct {
    char data[VSTR_SMALL_MAX_LEN];
} vstr_small;

typedef struct __attribute__((__packed__)) {
    char* data;
    uint64_t capacity;
    uint64_t length : 56;
} vstr_large;

typedef struct {
    union {
        vstr_small small;
        vstr_large large;
    } str_data;
    uint8_t is_large : 1;
    uint8_t small_available : 7;
} vstr;

vstr vstr_new(void);
vstr vstr_from(const char* init);
vstr vstr_from_length(const char* init, uint64_t length);
const char* vstr_data(const vstr* self);
uint64_t vstr_length(const vstr* self);
int vstr_cmp(const vstr* self, const vstr* other);
void vstr_push_char(vstr* self, char ch);
void vstr_cat_length(vstr* self, const char* string, uint64_t string_length);
void vstr_cat(vstr* self, const char* string);
void vstr_delete(vstr* self);

#endif /* __VSTR_H__ */
