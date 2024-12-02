#include "vstr.h"
#include "base.h"
#include <memory.h>

vstr vstr_new(void) {
    vstr self = {0};
    self.small_available = VSTR_SMALL_MAX_LEN;
    return self;
}

static vstr_large vstr_large_from_length(const char* init, uint64_t length) {
    vstr_large self = {0};
    aoc_assert(length <= VSTR_LARGE_MAX_LEN);
    uint64_t capacity = length + 1;
    self.data = vstr_malloc(capacity);
    aoc_assert(self.data != NULL);
    memcpy(self.data, init, length);
    self.data[length] = '\0';
    self.length = length;
    self.capacity = capacity;
    return self;
}

vstr vstr_from_length(const char* init, uint64_t length) {
    vstr self = {0};
    if (length > VSTR_SMALL_MAX_LEN) {
        self.is_large = 1;
        self.str_data.large = vstr_large_from_length(init, length);
        return self;
    }
    memcpy(self.str_data.small.data, init, length);
    self.small_available = VSTR_SMALL_MAX_LEN - length;
    return self;
}

vstr vstr_from(const char* init) {
    aoc_assert(init != NULL);
    return vstr_from_length(init, strlen(init));
}

const char* vstr_data(const vstr* self) {
    if (self->is_large) {
        return self->str_data.large.data;
    }
    return self->str_data.small.data;
}

uint64_t vstr_length(const vstr* self) {
    if (self->is_large) {
        return self->str_data.large.length;
    }
    return VSTR_SMALL_MAX_LEN - self->small_available;
}

int vstr_cmp(const vstr* self, const vstr* other) {
    const char* lhs = vstr_data(self);
    const char* rhs = vstr_data(other);
    uint64_t lhs_length = vstr_length(self);
    uint64_t rhs_length = vstr_length(other);

    if (lhs_length == rhs_length) {
        return strcmp(lhs, rhs);
    }

    int cmp = strncmp(lhs, rhs, lhs_length);
    if (cmp == 0) {
        if (lhs_length > rhs_length) {
            // lhs is larger than rhs
            return 1;
        } else {
            // rhs is larger than lhs
            return -1;
        }
    }
    return cmp;
}

static void vstr_large_push_char(vstr_large* self, char ch) {
    uint64_t length = self->length;
    uint64_t capacity = self->capacity;
    aoc_assert((length + 1) <= VSTR_LARGE_MAX_LEN);
    length += 1;
    if (length >= (capacity - 1)) {
        capacity <<= 1;
        void* mem = vstr_realloc(self->data, capacity);
        aoc_assert(mem != NULL);
        self->data = mem;
        self->capacity = capacity;
    }
    self->data[self->length] = ch;
    self->length = length;
    self->data[self->length] = '\0';
}

void vstr_push_char(vstr* self, char ch) {
    if (self->is_large) {
        vstr_large_push_char(&self->str_data.large, ch);
        return;
    }
    if (self->small_available == 0) {
        vstr_large large = vstr_large_from_length(self->str_data.small.data,
                                                  VSTR_SMALL_MAX_LEN);
        self->is_large = 1;
        self->str_data.large = large;
        vstr_large_push_char(&self->str_data.large, ch);
        return;
    }
    self->str_data.small.data[VSTR_SMALL_MAX_LEN - self->small_available] = ch;
    self->small_available -= 1;
}

static void vstr_large_cat_length(vstr_large* self, const char* string,
                                  uint64_t string_length) {
    uint64_t length = self->length;
    uint64_t capacity = self->capacity;
    uint64_t new_length = length + string_length;
    aoc_assert(new_length <= VSTR_LARGE_MAX_LEN);
    if (new_length >= (capacity - 1)) {
        capacity <<= 1;
        void* mem = vstr_realloc(self->data, capacity);
        aoc_assert(mem != NULL);
        self->data = mem;
        self->capacity = capacity;
    }
    memcpy(self->data + length, string, string_length);
    self->data[new_length] = '\0';
}

void vstr_cat_length(vstr* self, const char* string, uint64_t string_length) {
    if (self->is_large) {
        vstr_large_cat_length(&self->str_data.large, string, string_length);
        return;
    }
    uint64_t length = VSTR_SMALL_MAX_LEN - self->small_available;
    if (length + string_length > VSTR_SMALL_MAX_LEN) {
        vstr_large large =
            vstr_large_from_length(self->str_data.small.data, length);
        self->is_large = 1;
        self->str_data.large = large;
        vstr_large_cat_length(&self->str_data.large, string, string_length);
        return;
    }
    memcpy(self->str_data.small.data + length, string, string_length);
    self->small_available -= string_length;
}

void vstr_cat(vstr* self, const char* string) {
    vstr_cat_length(self, string, strlen(string));
}

void vstr_delete(vstr* self) {
    if (self->is_large) {
        vstr_free(self->str_data.large.data);
    }
}
