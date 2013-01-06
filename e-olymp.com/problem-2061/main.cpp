#include <iostream>

int nok(int a, int b) {
	int x = a;
	int y = b;
	while (a > 0 && b > 0) {
		if (a > b) {
			a = a % b;
		} else {
			b = b % a;
		}
	}
	return x * y / (a > b ? a : b);
}

int main(int argc, char **argv) {
	int x, y, n, m;
	std::cin >> x >> y >> n >> m;

	int l = nok(n, m);
	int i = 1;
	while ((l * i) / m + (l * i) / n < x || (l * i) / m < n || (l * i) / n < m)
		i++;

	std::cout << (l * i) / n + (l * i) / m << " " << (l * i) / n << " "
			<< (l * i) / m << std::endl;

	return 0;
}

