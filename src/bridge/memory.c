#include <stddef.h>

// Aligns memory pointers to 8-byte boundaries for Wasm stability
void* align_memory(void* ptr) {
    return (void*)(((uintptr_t)ptr + 7) & ~7);
}
