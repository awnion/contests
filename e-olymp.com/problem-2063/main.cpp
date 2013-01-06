#include <stdio.h>
#define BUFSIZE 10000

int main() {
	FILE *fin, *fout;
	int l;
	char zeros[BUFSIZE];
	for (l = 0; l < BUFSIZE; ++l)
		zeros[l] = '0';

	fin = fopen("input.txt", "r");
	fseek(fin, 0, SEEK_END);
	l = ftell(fin) - 2;

	fout = fopen("output.txt", "w");
	setvbuf(fout, NULL, _IOFBF, BUFSIZE);
	if (l == 1 || l == 2) {
		fprintf(fout, "%d\n", 45 * (l - 1) + 1);
		return 0;
	}

	fprintf(fout, "%d", 45 * (l - 1));

	l -= 3;

	while (l > BUFSIZE) {
		fwrite(zeros, 1, BUFSIZE, fout);
		l -= BUFSIZE;
	}

	for (int i = 0; i < l; ++i) {
		fprintf(fout, "0");
	}

	fprintf(fout, "1\n");
	return 0;
}

