#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main(int argc, char **argv) {
	int n, k;
	int a[1000];

	ifstream in("input.txt");
	ofstream out("output.txt");

	in >> n >> k;

	for (int i = 0; i < n; ++i) {
		in >> a[i];
		if (i + 1 >= k && a[i] > 0) {
			out << i + 1 << endl;
			in.close();
			out.close();

			return 0;
		}
	}

	for (int i = 0; i < n; ++i) {
		if (a[i] > 0) {
			out << i + 1 << endl;
			in.close();
			out.close();
			return 0;
		}
	}


	in.close();
	out.close();

	return 0;
}

