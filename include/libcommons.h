#ifndef LIBCOMMONS_H
#define LIBCOMMONS_H "0.7.0"

#ifndef LIBCOMMONS_PREFIX
#define LIBCOMMONS_PREFIX libcommons_
#endif // LIBCOMMONS_PREFIX

#ifndef LIBCOMMONS_ALLOCATOR
#define LIBCOMMONS_ALLOCATOR malloc
#endif // LIBCOMMONS_ALLOCATOR

#ifndef LIBCOMMONS_DEALLOCATOR
#define LIBCOMMONS_DEALLOCATOR free
#endif // LIBCOMMONS_DEALLOCATOR

#define __LIBCOMMONS_PREFIXED(A, B) A##B
#define _LIBCOMMONS_PREFIXED(A, B) __LIBCOMMONS_PREFIXED(A, B)
#define LIBCOMMONS_PREFIXED(A) _LIBCOMMONS_PREFIXED(LIBCOMMONS_PREFIX, A)

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/// UTF-8 owned string.
///
/// Libcommons strings are not null terminated.
typedef struct LIBCOMMONS_PREFIXED(ffi_string_t) {
    char *buf;
    uintptr_t len;
    uintptr_t capacity;
    void (*drop)(struct LIBCOMMONS_PREFIXED(ffi_string_t) *);
} LIBCOMMONS_PREFIXED(ffi_string_t);

/// Wide pointer to a UTF-8 string slice.
///
/// Libcommons strings are not null terminated.
typedef struct LIBCOMMONS_PREFIXED(ffi_str_p) {
    char *buf;
    uintptr_t len;
} LIBCOMMONS_PREFIXED(ffi_str_p);

// ffi_string_t

/// Allocate a new FFI string via a C string.
///
/// If string is empty, no allocation is performed.
///
/// Must be a valid UTF-8 string.
LIBCOMMONS_PREFIXED(ffi_string_t)
LIBCOMMONS_PREFIXED(ffi_string_new)(char *cstr);

/// Convert a string to a string slice.
LIBCOMMONS_PREFIXED(ffi_str_p)
LIBCOMMONS_PREFIXED(ffi_string_slice)(LIBCOMMONS_PREFIXED(ffi_string_t) *);

/// Append a C string to an FFI string.
///
/// Must be a valid UTF-8 string.
///
/// Returns 0 on success, -1 if allocation of a new string failed, -2 if
/// deallocation of the old string failed.
///
/// If allocation fails, string length is reset to what it was before.
int LIBCOMMONS_PREFIXED(ffi_string_push_c)(LIBCOMMONS_PREFIXED(ffi_string_t) *,
                                           char *cstr);

/// Append an FFI string slice to an FFI string.
///
/// Returns 0 on success, -1 if allocation of a new string failed, -2 if
/// deallocation of the old string failed.
///
/// If allocation fails, string length is reset to what it was before.
int LIBCOMMONS_PREFIXED(ffi_string_push_ffi)(
    LIBCOMMONS_PREFIXED(ffi_string_t) *, LIBCOMMONS_PREFIXED(ffi_str_p) str);

/// Free an FFI string.
///
/// Internal `drop` method must set all values to 0.
void LIBCOMMONS_PREFIXED(ffi_string_free)(LIBCOMMONS_PREFIXED(ffi_string_t) *);

// ffi_str_p

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
LIBCOMMONS_PREFIXED(ffi_str_p) LIBCOMMONS_PREFIXED(ffi_str_new)(char *cstr);

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
bool LIBCOMMONS_PREFIXED(ffi_str_eq)(LIBCOMMONS_PREFIXED(ffi_str_p) one,
                                     LIBCOMMONS_PREFIXED(ffi_str_p) other);

/// Create a substring from slice.
LIBCOMMONS_PREFIXED(ffi_str_p)
LIBCOMMONS_PREFIXED(ffi_str_substr)(LIBCOMMONS_PREFIXED(ffi_str_p) str,
                                    uintptr_t start, uintptr_t len);

#ifdef LIBCOMMONS_IMPLEMENTATION

void __libcommons_internal_string_t_drop(LIBCOMMONS_PREFIXED(ffi_string_t) *
                                         self) {
    if (self->buf != NULL)
        free(self->buf);
}

/// Allocate a new FFI string via a C string.
///
/// If string is empty, no allocation is performed.
///
/// Must be a valid UTF-8 string.
LIBCOMMONS_PREFIXED(ffi_string_t)
LIBCOMMONS_PREFIXED(ffi_string_new)(char *cstr) {
    uintptr_t len = strlen(cstr);

    if (len == 0) {
        LIBCOMMONS_PREFIXED(ffi_string_t)
        value = {
            .buf = NULL,
            .len = 0,
            .capacity = 0,
            .drop = NULL,
        };
        return value;
    }

    char *buf = (char *)LIBCOMMONS_ALLOCATOR(len);
    memcpy((void *)buf, (void *)cstr, len);
    LIBCOMMONS_PREFIXED(ffi_string_t)
    value = {
        .buf = buf,
        .len = len,
        .capacity = len,
        .drop = &__libcommons_internal_string_t_drop,
    };
    return value;
}

/// Convert a string to a string slice.
LIBCOMMONS_PREFIXED(ffi_str_p)
LIBCOMMONS_PREFIXED(ffi_string_slice)(LIBCOMMONS_PREFIXED(ffi_string_t) * str) {
    LIBCOMMONS_PREFIXED(ffi_str_p)
    value = {
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
int LIBCOMMONS_PREFIXED(ffi_string_push_c)(LIBCOMMONS_PREFIXED(ffi_string_t) *
                                               self,
                                           char *cstr) {
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
int LIBCOMMONS_PREFIXED(ffi_string_push_ffi)(LIBCOMMONS_PREFIXED(ffi_string_t) *
                                                 self,
                                             LIBCOMMONS_PREFIXED(ffi_str_p)
                                                 str) {
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
void LIBCOMMONS_PREFIXED(ffi_string_free)(LIBCOMMONS_PREFIXED(ffi_string_t) *
                                          self) {
    self->drop(self);
    memset(self, 0, sizeof(LIBCOMMONS_PREFIXED(ffi_string_t)));
}

// ffi_str_p

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
LIBCOMMONS_PREFIXED(ffi_str_p)
LIBCOMMONS_PREFIXED(ffi_str_new)(char *cstr) {
    uintptr_t len = strlen(cstr);

    if (len == 0) {
        LIBCOMMONS_PREFIXED(ffi_str_p)
        value = {
            .buf = NULL,
            .len = 0,
        };
        return value;
    }

    LIBCOMMONS_PREFIXED(ffi_str_p)
    value = {
        .buf = cstr,
        .len = len,
    };
    return value;
}

/// Create a new FFI string slice referencing a C string.
///
/// Must be a valid UTF-8 string and remain valid for the
/// entire duration of its usage.
bool LIBCOMMONS_PREFIXED(ffi_str_eq)(LIBCOMMONS_PREFIXED(ffi_str_p) one,
                                     LIBCOMMONS_PREFIXED(ffi_str_p) other) {
    if (one.len != other.len)
        return false;

    for (uintptr_t i = 0; i < one.len; i++)
        if (one.buf[i] != other.buf[i])
            return false;

    return true;
}

/// Create a substring from slice.
LIBCOMMONS_PREFIXED(ffi_str_p)
LIBCOMMONS_PREFIXED(ffi_str_substr)(LIBCOMMONS_PREFIXED(ffi_str_p) str,
                                    uintptr_t start, uintptr_t len) {
    if (start >= str.len) {
        LIBCOMMONS_PREFIXED(ffi_str_p)
        value = {
            .buf = NULL,
            .len = 0,
        };
        return value;
    }

    uintptr_t new_len = len;
    if (start + len > str.len)
        new_len = str.len - start;

    LIBCOMMONS_PREFIXED(ffi_str_p)
    value = {
        .buf = str.buf + start,
        .len = new_len,
    };
    return value;
}

#endif // LIBCOMMONS_IMPLEMENTATION

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // LIBCOMMONS_H
