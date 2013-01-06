#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int solve(int n) {
	return 1;
}

int main(int argc, char **argv) {
	int t, n;

	ifstream in("input.txt");
	ofstream out("output.txt");

	in >> t;

	for (int i = 0; i < t; ++i) {
		in >> n;
		out << (n + 1) % 2 << endl;
	}

	in.close();
	out.close();

	return 0;
}

