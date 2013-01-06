#include <iostream>
#include <fstream>
#include <string>

using namespace std;

bool check(int A, int B, int C, int a, int b, int c) {
	return A * b == B * a && C * b == B * c && A * c == C * a;
}

int main(int argc, char **argv) {
	int n, m;

	int a, b, c;

	int x[50][50] = { { 0 } };
	int y[50] = { 0 };
	int z[50] = { 0 };

	int count = 0;

	ifstream in("input.txt");
	ofstream out("output.txt");

	in >> n >> m;

	for (int i = 0; i < n; ++i)
		for (int j = 0; j < m; ++j)
			in >> x[i][j];

	in >> a >> b >> c;

	for (int i = 0; i < n; ++i) {
		for (int j = 0; j < m; ++j) {
			y[i] += x[i][j];
			z[j] += x[i][j];
		}
	}

	for (int i = 1; i < n; ++i) {
		y[i] += y[i - 1];
	}

	for (int i = 1; i < m; ++i) {
		z[i] += z[i - 1];
	}

	if (n >= 3) {
		for (int i = 1; i < n - 1; ++i) {
			for (int j = i + 1; j < n; ++j) {
				int A, B, C;
				A = y[i - 1];
				B = y[j - 1] - y[i - 1];
				C = y[n - 1] - y[j - 1];
				if (check(A, B, C, a, b, c) || check(A, C, B, a, b, c)
						|| check(B, A, C, a, b, c) || check(B, C, A, a, b, c)
						|| check(C, A, B, a, b, c) || check(C, B, A, a, b, c))
					count++;
			}
		}
	}

	if (m >= 3) {
		for (int i = 1; i < m - 1; ++i) {
			for (int j = i + 1; j < m; ++j) {
				int A, B, C;
				A = z[i - 1];
				B = z[j - 1] - z[i - 1];
				C = z[m - 1] - z[j - 1];
				if (check(A, B, C, a, b, c) || check(A, C, B, a, b, c)
						|| check(B, A, C, a, b, c) || check(B, C, A, a, b, c)
						|| check(C, A, B, a, b, c) || check(C, B, A, a, b, c))
					count++;
			}
		}
	}

	out << count << endl;

	in.close();
	out.close();

	return 0;
}

