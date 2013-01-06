#include <stdio.h>

#define MAXLEN 1000002

int main() {
	char *s = new char[MAXLEN + 4];
	int *p = new int[MAXLEN + 4];
//	FILE *fin, *fout;

//	fin = fopen("input.txt", "r");
	setvbuf(stdin, NULL, _IOFBF, 20000);
	fgets(s, MAXLEN + 2, stdin);

	p[0] = -1;
	int i = 1;

	while (s[i - 1] != 0) {
		p[i] = p[i - 1] + 1;
		while (s[i - 1] != s[p[i] - 1] && p[i] > 0)
			p[i] = p[p[i] - 1] + 1;
		i++;
	}

	printf("%d\n", s[i - 2] == '\n' ? p[i - 2] : p[i - 1]);

	return 0;
}
