#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int solve(int n) {
	return 1;
}

int main(int argc, char **argv) {
	int t, n;
	int a[100][100] = { { 0 } };
	int x, y;
	int result = 0;
	int max;

	ifstream in("input.txt");
	ofstream out("output.txt");

	in >> t;

	for (int i = 0; i < t; ++i) {
		in >> n;

		for (int m = 0; m < n; ++m)
			for (int k = 0; k < n; ++k)
				a[m][k] = k != m ? -1 : 0;

		for (int j = 0; j < n - 1; ++j) {
			in >> x >> y;
			a[x - 1][y - 1] = 1;
			a[y - 1][x - 1] = 1;
		}

		for (int m = 0; m < n; ++m) {
			for (int k = 0; k < n; ++k) {
				for (int l = 0; l < n; ++l) {
					if (a[k][m] != -1 && a[m][l] != -1
							&& (a[k][m] + a[m][l] < a[k][l] || a[k][l] == -1)) {
						a[k][l] = a[k][m] + a[m][l];
					}
				}
			}
		}

		max = 0;

		for (int m = 0; m < n; ++m)
			for (int k = 0; k < n; ++k)
				if (max < a[m][k])
					max = a[m][k];

		result += max;
	}

	out << result;

	in.close();
	out.close();

	return 0;
}

