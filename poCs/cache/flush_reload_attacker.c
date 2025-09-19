/* flush_reload_attacker.c
 * Minimal Flush+Reload timing loop for education.
 * Compile: gcc -O2 -o flush_reload_attacker flush_reload_attacker.c -march=native
 *
 * Reminder: real cross-process Flush+Reload needs a shared mapping (mmap of same file,
 * shared library) between victim and attacker. This code is a local timing demo.
 */
#include <stdio.h>
#include <stdint.h>
#include <x86intrin.h>
#include <unistd.h>

#define ITER 100000

static inline uint64_t timed_read(volatile char *addr) {
    unsigned int aux;
    uint64_t t1 = __rdtscp(&aux);
    volatile unsigned char v = *addr;
    uint64_t t2 = __rdtscp(&aux);
    (void)v;
    return t2 - t1;
}

int main() {
    static char probe[4096*256];
    volatile char *addr = &probe[4096*42];
    int hits = 0;
    for (int i=0;i<ITER;i++) {
        _mm_clflush((void*)addr);
        asm volatile("mfence":::"memory");
        usleep(100);
        uint64_t t = timed_read(addr);
        if (t < 200) hits++;
    }
    printf("hits: %d / %d\n", hits, ITER);
    return 0;
}
