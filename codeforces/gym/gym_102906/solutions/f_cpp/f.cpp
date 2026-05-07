#include <algorithm>
#include <cstring>
#include <iostream>

using namespace std;

int k, n, m, a[5005], b[5005], na[5005][5005], nb[5005][5005], dp[5005][5005],
    bt[5005][5005], btel[5005][5005];

int main() {
  ios::sync_with_stdio(0);
  cin.tie(0);
  cout.tie(0);
  //
  int ans = 0;
  cin >> k >> n;
  for (int i = 1; i <= n; i++)
    cin >> a[i];
  for (int i = k; i > 0; i--)
    na[n][i] = na[n + 1][i] = n + 1;
  for (int i = n - 1; i >= 0; i--) {
    for (int j = k; j > 0; j--)
      na[i][j] = na[i + 1][j];
    na[i][a[i + 1]] = i + 1;
  }
  cin >> m;
  for (int i = 1; i <= m; i++)
    cin >> b[i];
  for (int i = k; i > 0; i--)
    nb[m][i] = nb[m + 1][i] = m + 1;
  for (int i = m - 1; i >= 0; i--) {
    for (int j = k; j > 0; j--)
      nb[i][j] = nb[i + 1][j];
    nb[i][b[i + 1]] = i + 1;
  }

  if (k == 1) {
    ans = max(m, n) + 1;
    cout << ans << endl;
    for (int i = 0; i < ans; i++)
      cout << "1 ";
    return 0;
  }

  memset(dp, -1, sizeof dp);

  ans = 0;
  dp[0][0] = 0;
  int mm = 0;
  while (ans < 5005 && dp[ans][n + 1] < m + 1) {
    int prev = ans;
    ans++;

    for (int i = 0; i <= mm; i++) {
      int best_j = dp[prev][i];
      if (best_j > -1) {
        for (int el = k; el > 0; el--) {
          int next_a = na[i][el];
          if (next_a > mm)
            mm = next_a;
          int next_b = nb[best_j][el];
          if (next_b > dp[ans][next_a]) {
            dp[ans][next_a] = next_b;
            bt[ans][next_a] = i;
            btel[ans][next_a] = el;
          }
        }
      }
    }
  }

  cout << ans << endl;

  int r[5005];

  int best_i = n + 1;
  for (int l = ans; l > 0; --l) {
    r[l - 1] = btel[l][best_i];
    best_i = bt[l][best_i];
  }

  for (int i = 0; i < ans; ++i)
    cout << r[i] << " ";

  return 0;
}
