#include <stdio.h>

#define MAXLEN 1000002

int main() {
    char *s = new char[MAXLEN + 4];
    int *p = new int[MAXLEN + 4];
    int *a = new int[MAXLEN + 4];

    setvbuf(stdin, NULL, _IOFBF, 20000);
    setvbuf(stdout, NULL, _IOFBF, 20000);
    fgets(s, MAXLEN + 2, stdin);

    p[0] = -1;
    int i = 1;

    while (s[i - 1] != 0) {
        p[i] = p[i - 1] + 1;
        while (s[i - 1] != s[p[i] - 1] && p[i] > 0)
            p[i] = p[p[i] - 1] + 1;
        ++i;
    }

    if (s[i - 2] == '\n') --i;
    --i;

    int len = i;
    int k;
    while (i > 0) {
        k = p[i];
        while (k > 0) {
            if (!a[i - k]) a[i - k] = k;
            k = p[k];
        }
        --i;
    }

    for (i = 0; i < len - 1; i++) {
        printf("%d ", a[i]);
    }

    printf("%d\n", a[len - 1]);

    return 0;
}

