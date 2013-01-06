#include <iostream>
#include <fstream>
using namespace std;

int main(int argc, char **argv) {
	int n;
	int x[100001];
	int y[100001];
	int k[100001];
	int a, b;

	ifstream in("input.txt");
	ofstream out("output.txt");

	in >> n;

	for (int i = 0; i < n; ++i) {
		in >> a >> b;
		k[i] = a > 0 ? b > 0 ? 1 : 3 : b > 0 ? 2 : 4;
		x[i] = a > 0 ? a : -a;
		y[i] = b > 0 ? b : -b;
	}

	a = 0;
	b = 1;
	int ms = (x[a] - x[b]) * (x[a] - x[b]) + (y[a] - y[b]) * (y[a] - y[b]);
	int s;

	for (int i = 0; i < n - 1; ++i) {
		for (int j = i + 1; j < n; ++j) {
			s = (x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j]);
			if (s < ms) {
				ms = s;
				a = i;
				b = j;
			}
		}
	}

	out << a + 1 << " " << k[a] << " " << b + 1 << " " << 5 - k[b] << endl;

	in.close();
	out.close();

	return 0;
}

