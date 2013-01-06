#include <iostream>

using namespace std;

int main() {
    int m, n, k, r;
    cin >> m >> n >> k;

    r = m % n;
    m = m / n;

    cout << m;

    if (k > 0) {
        cout << ".";
        while (k > 0) {
            r *= 10;
            m = r / n;
            r = r % n;
            k--;
            cout << m;
        }
    }
    cout << endl;
}
