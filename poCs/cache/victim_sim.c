/* victim_sim.c
 * Minimal victim that accesses a probe array depending on a secret byte.
 * Compile: gcc -O2 -o victim_sim victim_sim.c
 *
 * NOTE: For cross-process Flush+Reload you would map a shared file or shared lib so
 * attacker and victim share the same physical pages. This simple demo uses a local buffer
 * to let you experiment with clflush/rdtscp timing mechanics.
 */
#include <stdio.h>
#include <unistd.h>
#include <stdint.h>

unsigned char probe[4096*256];
volatile unsigned char secret = 42;

int main() {
    for (int i=0;i<256;i++) probe[i*4096] = 1;
    while (1) {
        usleep(20000);
        volatile unsigned char tmp = probe[secret * 4096];
        (void)tmp;
    }
    return 0;
}
