#include <iostream>
#include <fstream>
#include <string>
#include <cmath>
#include <vector>
using namespace std;

typedef vector< int > row;
typedef vector< row > graph;

row &deixtra(graph g, int s, int n) {
    row path(n, -1);
    row visited(n, 0);
    path[s] = 0;
    visited[s] = 1;
    int current = s;
    do {
        for (int i = 0; i < n; i++) {
            if (i == current) continue;
            if (g[current][i] != -1 && 
                    (path[current] + g[current][i] < path[i] || 
                     path[i] == -1)) {
                path[i] = path[current] + g[current][i];
            }
        }

        int min = -1;
        int current = -1;
        for (int i = 0; i < n; i++) {
            if (min == -1 || path[i] < min) {

            }
        }
    } while (current != -1);
}

void test(ifstream &in, ofstream &out) {
    int n, m, e, b, p;
    in >> n >> m >> e;

    graph g(n, row(n, -1));

    int x, y, z;
    for (int i = 0; i < m; i++) {
        in >> x >> y >> z;
        g[x - 1][y - 1] = g[y - 1][x - 1] = z;
    }

    row exits(e, 0);

    for (int i = 0; i < e; i++) {
        in >> x;
        exits[i] = x - 1;
    }

    in >> b >> p;
    p--;
    b--;


    out << "IMPOSSIBLE" << endl;
}

int main(int argc, const char *argv[])
{
    ifstream in("input.txt");
    ofstream out("output.txt");
    test(in, out);
    return 0;
}

