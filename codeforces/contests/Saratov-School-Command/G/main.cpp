#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main(int argc, char **argv) {
	int t, n, m;
	int a1[100];
	int b1[100];
	int a2[100];
	int b2[100];
	int c[100];
	int d[100][100] = { { 0 } };
	string s[100];


	ifstream in("input.txt");
	ofstream out("output.txt");

	in >> n >> t;

	for (int i = 0; i < n; ++i) {
		in >> a1[i] >> b1[i] >> a2[i] >> b2[i];
	}

	in >> m;

	for (int i = 0; i < m; ++i) {
		in >> s[i];
		in >> c[i];
	}



	in.close();
	out.close();

	return 0;
}
