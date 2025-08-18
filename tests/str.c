#include <assert.h>
#define LIBCOMMONS_PREFIX
#define LIBCOMMONS_IMPLEMENTATION

#include "../include/libcommons.h"

int main() {
    ffi_string_t v = ffi_string_new("hello");
    assert(v.len == 5);

    ffi_str_p ptr = ffi_string_slice(&v);

    assert(ffi_str_eq(ptr, ffi_str_new("hello")));
    assert(!ffi_str_eq(ptr, ffi_str_new("hi")));

    ffi_string_free(&v);
}
