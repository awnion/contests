#include <iostream>

int main(int argc, char **argv) {
	int a, b, c, d, e;
	int x;
	std::cin >> a >> b >> c >> d >> e;
	int abcde = a * b * c * d * e;
	int y = a * b * c * d - b * c * d - a * c * d - a * b * d - a * b * c;

	if (abcde <= 0 || y <= 0) {
		std::cout << -1 << std::endl;
		return 0;
	}

	x = abcde / y;
	if (x % a == 0 && x % b == 0 && x % c == 0 && x % d == 0
			&& x - x / a - x / b - x / c - x / d == e) {
		std::cout << x << std::endl;
	} else {
		std::cout << -1 << std::endl;
	}

	return 0;
}

