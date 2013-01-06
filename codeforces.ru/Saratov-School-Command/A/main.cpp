#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main(int argc, char **argv) {
	int a;
	string s;

	ifstream in("input.txt");
	ofstream out("output.txt");

	in >> s >> a;

	if (((s == "front") && (a == 1)) || ((s == "back") && (a == 2))) {
		out << "L" << endl;
	} else {
		out << "R" << endl;
	}

	in.close();
	out.close();

	return 0;
}

