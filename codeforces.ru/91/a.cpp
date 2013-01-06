#include <iostream>

bool luck(int i) {
    while (i > 0) {
        if (i % 10 != 4 && i % 10 != 7) {
            return false;
        }
        i /= 10;
    }
    return true;
}

int main(int argc, const char *argv[])
{
    int n;
    std::cin >> n;
    for (int i = 1; i <= n; i++) {
        if (n % i == 0 && luck(i)) {
            std::cout << "YES" << std::endl;
            return 0;
        }
    }
    std::cout << "NO" << std::endl;
    return 0;
}
