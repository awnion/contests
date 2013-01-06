#include <iostream>

int p9[] = { 9, 99, 999, 9999, 99999, 999999, 9999999, 99999999, 999999999 };

inline int step(int n) {
	int sum = 0;
	n /= 10;
	int i = 0;
	while (n > 0) {
		sum += (n % 10) * p9[i];
		n /= 10;
		i++;
	}
	return sum;
}

inline int solve(int n) {
	int i = 0;

	while (n > 0) {
		n = step(n);
		i++;
	}

	return i;
}

int main() {
	int n;
	std::cin >> n;
	std::cout << solve(n) << std::endl;

//	for (int i = 1; i < 2000; ++i) {
////		int s = solve(i);
//		std::cout << i << " - " << solve(i) << std::endl;
//	}
	return 0;
}
