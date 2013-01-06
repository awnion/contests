#include <iostream>
#include <string>
#include <queue>

using namespace std;

// constants
enum {
    MAXN = 100
};

size_t a[MAXN][MAXN];
size_t b[MAXN][MAXN];

size_t dx[4] = {-1, 0, 1, 0};
size_t dy[4] = {0, -1, 0, 1};

size_t n, m;

void wave(size_t row, size_t column, size_t value) 
{
    queue<int> q_row;
    queue<int> q_column;
    
    size_t r, c;
    size_t new_r, new_c;
    
    q_row.push(row);
    q_column.push(column);
    
    while (!q_row.empty()) 
    {
        r = q_row.front();
        c = q_column.front();
        q_row.pop();
        q_column.pop();
        
        b[r][c] = value;
        
        for(size_t i = 0; i < 4; ++i)
        {
            new_c = c + dx[i];
            new_r = r + dy[i];
            if ((new_c >= 0) && 
                (new_c < n) && 
                (new_r >= 0) && 
                (new_r < m) && 
                (a[new_r][new_c] == 1) &&
                (b[new_r][new_c] == 0))
            {
                b[new_r][new_c] = value;
                q_row.push(new_r);
                q_column.push(new_c);
            }
        }
    }
}

int main (int argc, char const *argv[])
{
    string s;
    size_t result = 0;
    cin >> m >> n;
    
    for(size_t i = 0; i < m; ++i)
    {
        cin >> s;
        for(size_t j = 0; j < n; ++j)
        {
            a[i][j] = (s[j] == '#') ? 1 : 0;
        }
    }
    
    for(size_t i = 0; i < m; ++i)
    {   
        for(size_t j = 0; j < n; ++j)
        {
            if (a[i][j] == 1 && b[i][j] == 0) {
                result++;
                wave(i, j, result);
            }
        }
    }
    
    cout << result << endl;
    
    return 0;
}