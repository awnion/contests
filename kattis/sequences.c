#include <stdlib.h>
#include <stdio.h>

#define P 1000000007

int main(int argc, char *argv[]) {
    unsigned long long int r = 0;
    unsigned long long int q = 500000004;
    unsigned int o = 0;
    int c;

    c = fgetc(stdin);

    while(1) {
        if (c == '?') {
            r <<= 1;
            r += o * q % P;
            r %= P;

            q <<= 1;
            q %= P;

            o++;
        } else if (c == '0') {
            r += o * q % P;
            r %= P;
        } else if (c == '1') {
            o += 2;
        } else {
            break;
        }
        c = fgetc(stdin);
    }
    printf("%lld\n", r);
}
