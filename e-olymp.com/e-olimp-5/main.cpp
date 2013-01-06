#include <iostream>

using namespace std;

//int solve(int k)
//{
//    int n = 1;
//    int i;
//    for (i = 0; i < k; i++) n *= i + 1;

//    int j = 1;
//    while (n * j < i * i) j++;
//    return n * j;
//}

int solve(int k)
{
    int n = 1;
    int i;
    for (i = 0; i < k; i++) n *= i + 1;

    int j = 1;
    while (n * j < i * i) j++;
    return n * j;
}

int main()
{
    int k;
    cin >> k;
    cout << solve(k) << endl;
}
