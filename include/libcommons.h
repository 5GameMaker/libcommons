#ifndef LIBCOMMONS_H
#define LIBCOMMONS_H

#ifndef LIBCOMMONS_ALLOCATOR
#define LIBCOMMONS_ALLOCATOR malloc
#endif // LIBCOMMONS_ALLOCATOR

#ifndef LIBCOMMONS_DEALLOCATOR
#define LIBCOMMONS_DEALLOCATOR free
#endif // LIBCOMMONS_DEALLOCATOR

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#define LIBCOMMONS_LINK_SECTION(X)  __attribute__((section(X)))

/// UTF-8 owned string.
///
/// Libcommons strings are not null terminated.
typedef struct libcommons_ffi_string {
    char *buf;
    uintptr_t len;
    uintptr_t capacity;
    void (*drop)(struct libcommons_ffi_string *);
} libcommons_ffi_string;

/// Wide pointer to a UTF-8 string slice.
///
/// Libcommons strings are not null terminated.
typedef struct libcommons_ffi_str {
    char *buf;
    uintptr_t len;
} libcommons_ffi_str;

// ffi_string_t

/// Allocate a new FFI string via a C string.
///
/// If string is empty, no allocation is performed.
///
/// Must be a valid UTF-8 string.
libcommons_ffi_string libcommons_ffi_string_new(char *cstr);

/// Convert a string to a string slice.
libcommons_ffi_str libcommons_ffi_string_slice(libcommons_ffi_string *);

/// Append a C string to an FFI string.
///
/// Must be a valid UTF-8 string.
///
/// Returns 0 on success, -1 if allocation of a new string failed, -2 if
/// deallocation of the old string failed.
///
/// If allocation fails, string length is reset to what it was before.
int libcommons_ffi_string_push_c(libcommons_ffi_string *, char *cstr);

/// Append an FFI string slice to an FFI string.
///
/// Returns 0 on success, -1 if allocation of a new string failed, -2 if
/// deallocation of the old string failed.
///
/// If allocation fails, string length is reset to what it was before.
int libcommons_ffi_string_push_ffi(libcommons_ffi_string *, libcommons_ffi_str str);

/// Free an FFI string.
///
/// Internal `drop` method must set all values to 0.
void libcommons_ffi_string_free(libcommons_ffi_string *);

// ffi_str

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
libcommons_ffi_str libcommons_ffi_str_new(char *cstr);

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
bool libcommons_ffi_str_eq(libcommons_ffi_str one, libcommons_ffi_str other);

/// Create a substring from slice.
libcommons_ffi_str libcommons_ffi_str_substr(libcommons_ffi_str str, uintptr_t start, uintptr_t len);

/// Version of libcommons library.
///
/// This symbol is not affected by LIBCOMMONS_STRIP_PREFIX.
static const libcommons_ffi_str libcommons_version = {
    .buf = (char *) "0.8.0",
    .len = 5,
};

#ifdef LIBCOMMONS_IMPLEMENTATION

void __libcommons_internal_string_t_drop(libcommons_ffi_string *self) {
    if (self->buf != NULL)
        free(self->buf);
}

/// Allocate a new FFI string via a C string.
///
/// If string is empty, no allocation is performed.
///
/// Must be a valid UTF-8 string.
libcommons_ffi_string libcommons_ffi_string_new(char *cstr) {
    uintptr_t len = strlen(cstr);

    if (len == 0) {
        libcommons_ffi_string value = {
            .buf = NULL,
            .len = 0,
            .capacity = 0,
            .drop = NULL,
        };
        return value;
    }

    char *buf = (char *)LIBCOMMONS_ALLOCATOR(len);
    memcpy((void *)buf, (void *)cstr, len);
    libcommons_ffi_string value = {
        .buf = buf,
        .len = len,
        .capacity = len,
        .drop = &__libcommons_internal_string_t_drop,
    };
    return value;
}

/// Convert a string to a string slice.
libcommons_ffi_str libcommons_ffi_string_slice(libcommons_ffi_string *str) {
    libcommons_ffi_str value = {
        .buf = str->buf,
        .len = str->len,
    };
    return value;
}

/// Append a C string to an FFI string.
///
/// Must be a valid UTF-8 string.
///
/// Returns 0 on success, -1 if allocation of a new string failed, -2 if
/// deallocation of the old string failed, -3 if string length has overflowed.
///
/// If allocation fails, string length is reset to what it was before.
int libcommons_ffi_string_push_c(libcommons_ffi_string *self, char *cstr) {
    uintptr_t cstrlen = strlen(cstr) - 1;
    if (cstrlen == 0)
        return 0;

    if (self->len + cstrlen <= self->capacity) {
        memcpy(self->buf + self->len, cstr, cstrlen);
        self->len += cstrlen;
        return 0;
    }

    uintptr_t newsize = self->capacity;
    if (newsize == 0)
        newsize = 1;
    while (newsize < self->len + cstrlen) {
        uintptr_t old;
        newsize *= 2;
        if (newsize < old)
            return -3;
    }

    char *newbuf = (char *)LIBCOMMONS_ALLOCATOR(newsize);
    if (newbuf == NULL)
        return -1;
    if (self->buf != NULL) {
        memcpy(newbuf, self->buf, self->len);
        self->drop(self);
    }

    memcpy(newbuf + self->len, cstr, cstrlen);

    self->capacity = newsize;
    self->buf = newbuf;
    self->len += cstrlen;
    self->drop = &__libcommons_internal_string_t_drop;

    return 0;
}

/// Append an FFI string slice to an FFI string.
///
/// Returns 0 on success, -1 if allocation of a new string failed, -2 if
/// deallocation of the old string failed.
///
/// If allocation fails, string length is reset to what it was before.
int libcommons_ffi_string_push_ffi(
    libcommons_ffi_string * self, libcommons_ffi_str str) {
    if (str.len == 0)
        return 0;

    if (self->len + str.len <= self->capacity) {
        memcpy(self->buf + self->len, str.buf, str.len);
        self->len += str.len;
        return 0;
    }

    uintptr_t newsize = self->capacity;
    if (newsize == 0)
        newsize = 1;
    while (newsize < self->len + str.len) {
        uintptr_t old;
        newsize *= 2;
        if (newsize < old)
            return -3;
    }

    char *newbuf = (char *)LIBCOMMONS_ALLOCATOR(newsize);
    if (newbuf == NULL)
        return -1;
    if (self->buf != NULL) {
        memcpy(newbuf, self->buf, self->len);
        self->drop(self);
    }

    memcpy(newbuf + self->len, str.buf, str.len);

    self->capacity = newsize;
    self->buf = newbuf;
    self->len += str.len;
    self->drop = &__libcommons_internal_string_t_drop;

    return 0;
}

/// Free an FFI string.
///
/// Will call the internal `drop` method and set all values to 0.
void libcommons_ffi_string_free(libcommons_ffi_string *self) {
    self->drop(self);
    memset(self, 0, sizeof(libcommons_ffi_string));
}

// ffi_str

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
libcommons_ffi_str libcommons_ffi_str_new(char *cstr) {
    uintptr_t len = strlen(cstr);

    if (len == 0) {
        libcommons_ffi_str value = {
            .buf = NULL,
            .len = 0,
        };
        return value;
    }

    libcommons_ffi_str value = {
        .buf = cstr,
        .len = len,
    };
    return value;
}

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
bool libcommons_ffi_str_eq(libcommons_ffi_str one, libcommons_ffi_str other) {
    if (one.len != other.len)
        return false;

    for (uintptr_t i = 0; i < one.len; i++)
        if (one.buf[i] != other.buf[i])
            return false;

    return true;
}

/// Create a substring from slice.
libcommons_ffi_str libcommons_ffi_str_substr(libcommons_ffi_str str, uintptr_t start, uintptr_t len) {
    if (start >= str.len) {
        libcommons_ffi_str value = {
            .buf = NULL,
            .len = 0,
        };
        return value;
    }

    uintptr_t new_len = len;
    if (start + len > str.len)
        new_len = str.len - start;

    libcommons_ffi_str value = {
        .buf = str.buf + start,
        .len = new_len,
    };
    return value;
}

#endif // LIBCOMMONS_IMPLEMENTATION

#ifdef LIBCOMMONS_STRIP_PREFIX

#define LINK_SECTION(X) LIBCOMMONS_LINK_SECTION(X)

typedef libcommons_ffi_string ffi_string;
typedef libcommons_ffi_str ffi_str;

// ffi_string_t

/// Allocate a new FFI string via a C string.
///
/// If string is empty, no allocation is performed.
///
/// Must be a valid UTF-8 string.
static inline ffi_string ffi_string_new(char *cstr) { return libcommons_ffi_string_new(cstr); }

/// Convert a string to a string slice.
static inline libcommons_ffi_str ffi_string_slice(libcommons_ffi_string *self) {
    return libcommons_ffi_string_slice(self);
}

/// Append a C string to an FFI string.
///
/// Must be a valid UTF-8 string.
///
/// Returns 0 on success, -1 if allocation of a new string failed, -2 if
/// deallocation of the old string failed.
///
/// If allocation fails, string length is reset to what it was before.
static inline int ffi_string_push_c(libcommons_ffi_string *self, char *cstr) {
    return libcommons_ffi_string_push_c(self, cstr);
}

/// Append an FFI string slice to an FFI string.
///
/// Returns 0 on success, -1 if allocation of a new string failed, -2 if
/// deallocation of the old string failed.
///
/// If allocation fails, string length is reset to what it was before.
static inline int ffi_string_push_ffi(libcommons_ffi_string *self, libcommons_ffi_str str) {
    return libcommons_ffi_string_push_ffi(self, str);
}

/// Free an FFI string.
///
/// Internal `drop` method must set all values to 0.
static inline void ffi_string_free(libcommons_ffi_string *self) {
    return libcommons_ffi_string_free(self);
}

// ffi_str

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
static inline libcommons_ffi_str ffi_str_new(char *cstr) {
    return libcommons_ffi_str_new(cstr);
}

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
static inline bool ffi_str_eq(libcommons_ffi_str one, libcommons_ffi_str other) {
    return libcommons_ffi_str_eq(one, other);
}

/// Create a substring from slice.
static inline libcommons_ffi_str ffi_str_substr(libcommons_ffi_str str, uintptr_t start, uintptr_t len) {
    return libcommons_ffi_str_substr(str, start, len);
}

#endif // LIBCOMMONS_STRIP_PREFIX

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // LIBCOMMONS_H
