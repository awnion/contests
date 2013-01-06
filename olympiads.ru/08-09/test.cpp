#include <fstream>
#include <math.h>

using namespace std;

int main(){
    
    double n;
    n = ceil(3/3);
    ofstream fout ("e.out");
    
    fout << n;
    
    fout.close();
    
    return 0;
}
