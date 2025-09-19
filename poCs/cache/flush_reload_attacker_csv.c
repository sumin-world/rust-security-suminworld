// flush_reload_attacker_csv.c
// Compile: gcc -O2 -o flush_reload_attacker_csv flush_reload_attacker_csv.c -march=native
#include <stdio.h>
#include <stdint.h>
#include <x86intrin.h>
#include <unistd.h>

#define ITER 20000

static inline uint64_t timed_read(volatile char *addr) {
    unsigned int aux;
    uint64_t t1 = __rdtscp(&aux);
    volatile unsigned char v = *addr;
    uint64_t t2 = __rdtscp(&aux);
    (void)v;
    return t2 - t1;
}

int main(void) {
    static char probe[4096*256];
    volatile char *addr = &probe[4096*42];

    // CSV 헤더
    printf("iter,cycles\n");

    for (int i = 0; i < ITER; i++) {
        _mm_clflush((void*)addr);                 // 캐시에서 해당 라인 flush
        asm volatile("mfence" ::: "memory");      // 순서 보장 fence
        usleep(100);                               // victim이 접근할 시간 약간 부여(100µs)

        uint64_t t = timed_read(addr);            // 접근 시간 측정
        printf("%d,%lu\n", i, (unsigned long)t);  // CSV 라인 출력
    }
    return 0;
}
