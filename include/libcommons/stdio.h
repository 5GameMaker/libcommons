#ifndef LIBCOMMONS_STDIO_H
#define LIBCOMMONS_STDIO_H

#include "../libcommons.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include <stdio.h>

size_t libcommons_ffi_str_write(libcommons_ffi_str str, FILE *file);

bool libcommons_ffi_str_write_all(libcommons_ffi_str str, FILE *file);

#ifdef LIBCOMMONS_IMPLEMENTATION

size_t libcommons_ffi_str_write(libcommons_ffi_str str, FILE *file) {
    return fwrite(str.buf, 1, (size_t)str.len, file);
}

bool libcommons_ffi_str_write_all)(libcommons_ffi_str str, FILE *file) {
    size_t p = 0;
    while (p < (size_t)str.len) {
        libcommons_ffi_str substr = libcommons_ffi_str_substr(str, p, str.len);
        size_t written = libcommons_ffi_str_write(substr, file);

        if (written == 0)
            return false;
        p += written;
    }

    return true;
}

#endif // LIBCOMMONS_IMPLEMENTATION

#ifdef LIBCOMMONS_STRIP_PREFIX

static inline size_t ffi_str_write(libcommons_ffi_str str, FILE *file) {
    return libcommons_ffi_str_write(str, file);
}

static inline bool ffi_str_write_all(libcommons_ffi_str str, FILE *file) {
    return libcommons_ffi_str_write_all(str, file);
}

#endif // LIBCOMMONS_STRIP_PREFIX

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // LIBCOMMONS_STDIO_H
