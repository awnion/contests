#include <iostream>

using namespace std;

int solve2d(int n)
{
    if (n == 0) return 0;
    int i = 1;
    while (i*i < n) i++;
    if (i*(i-1) < n) return n * 3 + 4 * i + 1;
    return n * 3 + 4 * i - 1;
}

int f(int n, int a, int b, int c)
{
    return (a+1)*(b+1)*c +
           (a+1)*b*(c+1) +
           a*(b+1)*(c+1) +
           solve2d(n - a * b * c);
}

int solve3d(int n)
{
    int i = 1;
    while (i*i*i <= n) i++;
    if (i*i*(i-1) < n)
        return f(n, i, i, i - 1);
    if (i*(i-1)*(i-1) < n)
        return f(n, i, i - 1, i - 1);
    return f(n, i - 1, i - 1, i - 1);
}

int main()
{
    int n;
    cin >> n;
    cout << solve3d(n) << endl;
}
