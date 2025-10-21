#ifndef LIBCOMMONS_STDIO_H
#define LIBCOMMONS_STDIO_H "0.7.0"

#include "libcommons.h"

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#include <stdio.h>

size_t LIBCOMMONS_PREFIXED(ffi_str_write)(LIBCOMMONS_PREFIXED(ffi_str_p) str,
                                          FILE *file);

bool LIBCOMMONS_PREFIXED(ffi_str_write_all)(LIBCOMMONS_PREFIXED(ffi_str_p) str,
                                            FILE *file);

#ifdef LIBCOMMONS_IMPLEMENTATION

size_t LIBCOMMONS_PREFIXED(ffi_str_write)(LIBCOMMONS_PREFIXED(ffi_str_p) str,
                                          FILE *file) {
    return fwrite(str.buf, 1, (size_t)str.len, file);
}

bool LIBCOMMONS_PREFIXED(ffi_str_write_all)(LIBCOMMONS_PREFIXED(ffi_str_p) str,
                                            FILE *file) {
    size_t p = 0;
    while (p < (size_t)str.len) {
        LIBCOMMONS_PREFIXED(ffi_str_p)
        substr = LIBCOMMONS_PREFIXED(ffi_str_substr)(str, p, str.len);
        size_t written = LIBCOMMONS_PREFIXED(ffi_str_write)(substr, file);

        if (written == 0)
            return false;
        p += written;
    }

    return true;
}

#endif // LIBCOMMONS_IMPLEMENTATION

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // LIBCOMMONS_STDIO_H
