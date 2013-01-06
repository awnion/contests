#include <iostream>
#include <cmath>

using namespace std;

double vabs(int a, int b, int c) {
	return sqrt(a * a + b * b + c * c);
}

double s(int a[], int b[], int c[]) {
	return 0.5 * vabs(
		(a[1] - c[1]) * (b[2] - c[2]) - (a[2] - c[2]) * (b[1] - c[1]),
		(a[2] - c[2]) * (b[0] - c[0]) - (a[0] - c[0]) * (b[2] - c[2]),
		(a[0] - c[0]) * (b[1] - c[1]) - (a[1] - c[1]) * (b[0] - c[0])
	);
}

int main() {
	int a[4][3];
	for (int i = 0; i < 4; i++) 
		for (int j = 0; j < 3; j++) 
			cin >> a[i][j];
	cout.precision(1);
	cout.setf(ios::fixed, ios::floatfield);
	cout << s(a[0], a[1], a[2]) + 
		s(a[0], a[1], a[3]) + 
		s(a[0], a[2], a[3]) + 
		s(a[1], a[2], a[3]) << endl;
}

