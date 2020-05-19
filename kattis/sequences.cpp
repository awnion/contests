#include <stdio.h>

#define P 1000000007

int main(int argc, char *argv[]) {
    char c = getchar();
    unsigned long long int a = 0;
    unsigned long long int q = 500000004;
    unsigned int o = 0;
    while (c) {
        if (c == '?') {
            a <<= 1;
            a += o * q % P;
            a %= P;

            q <<= 1;
            q %= P;

            o++;
        } else if (c == '0') {
            a += o * q % P;
            a %= P;
        } else if (c == '1') {
            o += 2;
        } else {
            break;
        }
        c = getchar();
    }
    printf("%lld\n", a);
}
