#include <iostream>
#include <string>
#include <map>

int main(int argc, char **argv) {
	char set[] = { 'M', 'D', 'C', 'L', 'X', 'V', 'I' };

	std::string s;
	std::cin >> s;

	std::map<char, int> rome_numbers;
	rome_numbers['I'] = 1;
	rome_numbers['V'] = 5;
	rome_numbers['X'] = 10;
	rome_numbers['L'] = 50;
	rome_numbers['C'] = 100;
	rome_numbers['D'] = 500;
	rome_numbers['M'] = 1000;

	int sum = 0;
	bool flag = false;
	char level = 'I';

	for (int i = s.length() - 1; i >= 0; --i) {
		if (s[i] == '+') {
			flag = false;
			level = 'I';
			continue;
		}

		flag = rome_numbers[s[i]] < rome_numbers[level];
		if (!flag)
			level = s[i];

		sum += rome_numbers[s[i]] * (flag ? -1 : 1);
	}

	s = "";

//	std::cout << sum << std::endl;

	for (int i = 0; i < 7; ++i) {
		while (rome_numbers[set[i]] <= sum) {
			sum -= rome_numbers[set[i]];
			s += set[i];
		}
		if (i == 6) continue;
		if (i % 2 == 0) {
			if (rome_numbers[set[i]] - rome_numbers[set[i + 2]] <= sum) {
				sum -= rome_numbers[set[i]] - rome_numbers[set[i + 2]];
				s += set[i + 2];
				s += set[i];
			}
		} else {
			if (rome_numbers[set[i]] - rome_numbers[set[i + 1]] <= sum) {
				sum -= rome_numbers[set[i]] - rome_numbers[set[i + 1]];
				s += set[i + 1];
				s += set[i];
			}
		}
	}

//	std::cout << sum << std::endl;
	std::cout << s << std::endl;

	return 0;
}

