#include <fstream>

using namespace std;

int main(){
    
    long long n;
    n = 50 * 10000 * 10000;
    n = n - 1;
    n = n + 10000;
    
    n = (long long)((long long)((long long)n - 1 + 10000) / (long long)10000) * (long long)10000 + 7;
    ofstream fout ("e.out");
    
    fout << n;
    
    fout.close();
    
    return 0;
}
