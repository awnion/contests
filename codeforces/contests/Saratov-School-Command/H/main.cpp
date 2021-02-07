#include <iostream>
#include <fstream>
#include <string>
#include <set>

using namespace std;

int h[650001] = { 0 };
int d[] = { 1, 29, 29 * 29, 29 * 29 * 29 };

string get_str(int r) {
	string s;
	while (r > 0) {
		s += (char) (r % 29 - 1) + 'a';
		r /= 29;
	}
	return s;
}

int get_min() {
	int min = 201;
	int mini = 0;
	for (int i = 650000; i > 0; --i) {
		if (h[i] > 0 && h[i] < min) {
			min = h[i];
			mini = i;
		}
	}
	return mini;
}

set<int> hash(string str) {
	set<int> result;
	size_t len = str.size();
	int a[10];
	for (size_t i = 0; i < len; ++i) {
		a[i] = (int) (str[i] - 'a') + 1;
	}
	int hh;
	if (len > 3)
		for (size_t i = 0; i < len - 3; ++i) {
			for (size_t j = i + 1; j < len - 2; ++j) {
				for (size_t k = j + 1; k < len - 1; ++k) {
					for (size_t m = k + 1; m < len; ++m) {
						hh = a[i] * d[0] + a[j] * d[1] + a[k] * d[2]
								+ a[m] * d[3];
						if (result.count(hh) == 0) {
							h[hh]++;
							result.insert(hh);
						}
					}
				}
			}
		}
	if (len > 2)
		for (size_t i = 0; i < len - 2; ++i) {
			for (size_t j = i + 1; j < len - 1; ++j) {
				for (size_t k = j + 1; k < len; ++k) {
					hh = a[i] * d[0] + a[j] * d[1] + a[k] * d[2];
					if (result.count(hh) == 0) {
						h[hh]++;
						result.insert(hh);
					}
				}
			}
		}
	if (len > 1)
		for (size_t i = 0; i < len - 1; ++i) {
			for (size_t j = i + 1; j < len; ++j) {
				hh = a[i] * d[0] + a[j] * d[1];
				if (result.count(hh) == 0) {
					h[hh]++;
					result.insert(hh);
				}
			}
		}
	return result;
}

int main(int argc, char **argv) {
	int n;
	string str;
	set<int> s[200];
	int result[200] = { 0 };

	ifstream in("input.txt");
	ofstream out("output.txt");

	in >> n;
	for (int i = 0; i < n; ++i) {
		in >> str;
		s[i] = hash(str);
	}

	int min;
	size_t max;
	int maxi;

	for (int i = 0; i < n; ++i) {
		min = get_min();
		if (min == 0) {
			out << "-1" << endl;
			in.close();
			out.close();
			return 0;
		}
		h[min] = 0;
		max = 0;
		for (int j = 0; j < n; ++j) {
			if (result[j] == 0)
				if (s[j].count(min) > 0) {
					if (s[j].size() > max) {
						max = s[j].size();
						maxi = j;
					}
					s[j].erase(min);
				}
		}
		result[maxi] = min;
		set<int>::iterator it;
		for (it=s[maxi].begin(); it!=s[maxi].end(); ++it) {
			h[*it]--;
		}
	}

	for (int i = 0; i < n; ++i) {
		out << get_str(result[i]) << endl;
	}

	in.close();
	out.close();

	return 0;
}

