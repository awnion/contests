#include <iostream>
#include <sstream>
#include <fstream>
#include <string>

int primes[200] = { 0 };

int binsearch(int x) {
	int a = 1;
	int b = primes[0] - 1;
	int c = (a + b) / 2;
	while (primes[c] != x) {
		if (primes[c] > x) {
			b = c - 1;
		} else {
			a = c + 1;
		}
		c = (a + b) / 2;
	}
	return c;
}

bool is_prime(int n) {
	if (n < 2) return false;
	int i = 2;
	while (i * i <= n) {
		if (n % i == 0)
			return false;
		i++;
	}
	return true;
}

void makeprimes() {
	primes[0] = 1;
	for (int i = 2; i < 1001; ++i) {
		if (is_prime(i)) {
			primes[primes[0]] = i;
			primes[0]++;
		}
	}
}

std::string solve(int n, int k) {
	std::ostringstream oss;

	int x = n;
	int q[200] = { 0 };

	while (x > 0) {
		int i = 1;
		if (is_prime(x)) {
			i = binsearch(x);
			if (i > q[0])
				q[0] = i;
			q[i] += 1;
		} else {
			int y = x;
			while (primes[i] <= y && i < primes[0]) {
				while (y % primes[i] == 0) {
					y /= primes[i];
					q[i]++;
				}
				if (q[i] > 0 && i > q[0])
					q[0] = i;
				i++;
			}
		}
		x -= k;
	}

	long long result = 1;
	for (int i = 1; i < q[0] + 1; ++i) {
		if (q[i] > 0) {
			result *= q[i] + 1;
		}

		if (result > 1e18)
			return std::string("oo");

	}

//	oss.setf(std::ios::fixed, std::ios::floatfield);
//	oss.precision(0);
	oss << result;
	return oss.str();
}

int main(int argc, char **argv) {
	std::ifstream fin("input.txt");
	std::ofstream fout("output.txt");

	int count;
//	std::cin >> count;
	fin >> count;

	makeprimes();

	std::string s;
	for (int sample = 1; sample <= count; ++sample) {
//		std::cin >> s;
		fin >> s;
		int k = 0;
		int n = 0;
		for (size_t i = 0; i < s.length(); ++i) {
			if (s[i] == '!') {
				k++;
			} else {
				n = n * 10 + (int) (s[i] - '0');
			}
		}
//		std::cout << "Sample " << sample << ": " << solve(n, k) << std::endl;
		fout << "Sample " << sample << ": " << solve(n, k) << std::endl;
	}

	return 0;
}
