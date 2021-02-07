#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main(int argc, char **argv) {
	int n, k;
	int a;

	int p = 0;

	ifstream in("input.txt");
	ofstream out("output.txt");

	in >> n >> k;

	for (int i = 0; i < n; ++i) {
		in >> a;
		int j = 0;
		while (a >= k && j < 3) {
			a -= k;
			j++;
		}
		p += a;
	}

	out << p << endl;

	in.close();
	out.close();

	return 0;
}

