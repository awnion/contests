// #include <bits/stdc++.h>
#include <stdio.h>
#include <cstring>
#include <algorithm>

using namespace std;

// #define pb push_back
// #define pf push_front
// #define pob pop_back
// #define pof pop_front
// #define mp make_pair
// #define fi first
// #define se second

typedef long long lli;
// typedef pair<int, int> ii;
// typedef pair<lli, lli> ll;

int k, n, m, a[5005], b[5005], na[5005][5005], nb[5005][5005], memo[5005][5005];

int dp(int x, int y) {
	if ((x > n) && (y > m))
		return 0;
	if (memo[x][y] != -1)
		return memo[x][y];
	int ret = 5005;
	for (int i = k; i > 0; i--)
		ret = min(ret, 1 + dp(na[x][i], nb[y][i]));
	memo[x][y] = ret;
	return ret;
}

void backtrack(int x, int y, int ans) {
	if (ans == 0)
		return;
	for (int i = k; i > 0; i--)
		if (ans - 1 == dp(na[x][i], nb[y][i])) {
			printf("%d", i);
			putchar((ans > 1) ? ' ' : '\n');
			backtrack(na[x][i], nb[y][i], ans - 1);
			return;
		}
}

int main() {
	int ans;
	scanf("%d %d", &k, &n);
	for (int i = 1; i <= n; i++)
		scanf("%d", &a[i]);
	for (int i = k; i > 0; i--)
		na[n][i] = na[n + 1][i] = n + 1;
	for (int i = n - 1; i >= 0; i--) {
		for (int j = k; j > 0; j--)
			na[i][j] = na[i + 1][j];
		na[i][a[i + 1]] = i + 1;
	}
	scanf("%d", &m);
	for (int i = 1; i <= m; i++)
		scanf("%d", &b[i]);
	for (int i = k; i > 0; i--)
		nb[m][i] = nb[m + 1][i] = m + 1;
	for (int i = m - 1; i >= 0; i--) {
		for (int j = k; j > 0; j--)
			nb[i][j] = nb[i + 1][j];
		nb[i][b[i + 1]] = i + 1;
	}
	memset(memo, -1, sizeof memo);
	ans = dp(0, 0);
	printf("%d\n", ans);
	backtrack(0, 0, ans);
	return 0;
}
