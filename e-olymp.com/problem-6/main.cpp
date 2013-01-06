/**
 * Путёвки
 *    Туристическая фирма не успела из-за больших морозов продать n (n < 15)
 *    путёвок на горнолыжные базы, срок действия которых уже наступил. С целью
 *    уменьшения убытков, было решено с 1 февраля все такие путёвки, которым
 *    осталось dk (dk ≤ 30) дней, продавать по номинальной стоимости –
 *    по сk (сk ≤ 100) грн за день только за те дни, что остались со дня
 *    продажи (k = 1..n).
 *
 *    На какую наибольшую сумму можно реализовать эти путёвки, если каждый
 *    день продавать по одной путёвке?
 *
 * Входные данные
 *    Первая строка содержит количество путёвок n. Каждая из следующих n
 *    строк содержит два числа – количество дней dk и стоимость дня ck.
 *
 * Выходные даные
 *    Максимальная сумма прибыли.
 */

#include <iostream>
#include <algorithm>

int n;
int **a;
int *cache;
int *e2;

inline int f(int level, int set) {
	if (cache[set] == -1) {
		for (int i = 0; i < n; ++i) {
			if ((e2[i] & set) != 0) {
				cache[set] = std::max(cache[set],
						f(level - 1, set ^ e2[i]) + a[i][level]);
			}
		}
	}
	return cache[set];
}

int solve(int *d, int *c) {
	int *array = new int[n * n];
	a = new int*[n];
	for (int i = 0; i < n; ++i) {
		a[i] = array + i * n;
		for (int j = 0; j < n; ++j)
			a[i][j] = std::max((d[i] - j) * c[i], 0);
	}

	// make some bitmap optimisation with cache :P
	cache = new int[1 << n];
	for (int i = 0; i < 1 << n; ++i) cache[i] = -1;
	cache[0] = 0;

	e2 = new int[n];
	for (int i = 0; i < n; ++i) e2[i] = 1 << i;

	int result = f(n - 1, (1 << n) - 1);

	delete[] a;
	delete[] array;
	delete[] cache;
	delete[] e2;

	return result;
}

int main(int argc, char **argv) {
	std::cin >> n;

	int *d = new int[n];
	int *c = new int[n];

	for (int i = 0; i < n; ++i)
		std::cin >> d[i] >> c[i];

	std::cout << solve(d, c) << std::endl;

	delete[] d;
	delete[] c;

	return 0;
}
