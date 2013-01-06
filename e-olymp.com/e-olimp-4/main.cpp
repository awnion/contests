#include <iostream>

using namespace std;

#define eps 0.00000001

bool eq(double a, double b)
{
    return (a - b < eps) && (a - b > -eps);
}

int solve(double x1, double y1, double r1, double x2, double y2, double r2)
{
    if (eq(x1, x2) && eq(y1, y2) && eq(r1, r2)) return -1;
    double r = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
    double rr1 = (r1 - r2) * (r1 - r2);
    double rr2 = (r1 + r2) * (r1 + r2);

    if (eq(r, rr1) || eq(r, rr2)) return 1;
    if (r > rr1 && r < rr2) return 2;
    return 0;
}

int main()
{
    double x1, y1, r1, x2, y2, r2;
    cin >> x1 >> y1 >> r1 >> x2 >> y2 >> r2;
    cout << solve(x1, y1, r1, x2, y2, r2) << endl;
}
