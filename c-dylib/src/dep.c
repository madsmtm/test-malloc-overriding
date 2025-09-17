#define _GNU_SOURCE
#include <stdlib.h>
#include <stdio.h>
#include <dlfcn.h>

const char* c_dylib_lookup_malloc_address(void) {
    Dl_info info;
    if (!dladdr((void *)malloc, &info)) {
        printf("failed finding `malloc`\n");
        abort();
    }
    return info.dli_fname;
}

void* c_dylib_malloc(size_t size) {
    return malloc(size);
}

void c_dylib_free(void* ptr) {
    free(ptr);
}
