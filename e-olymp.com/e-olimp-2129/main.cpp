#include <iostream>
#include <cmath>
using namespace std;

int main() {
	int x, y;
	cin >> x >> y;
	double result = atan2(y, x);
	if (result < 0) result += asin(1) * 4;
	cout.precision(6);
	cout.setf(ios::fixed, ios::floatfield);
	cout << result << endl;
}
