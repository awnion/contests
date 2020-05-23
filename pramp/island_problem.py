from typing import List
from queue import Queue

def get_number_of_islands(a: List[List[int]]) -> int:
    q = Queue()
    m, n = len(a), len(a[0])
    res = 0
    
    for i in range(m):
        for j in range(n):
            if a[i][j] == 1:
                q.put((i, j))
                res += 1
                while not q.empty():
                    x, y = q.get()
                    a[x][y] = 0
                    if x > 0 and a[x - 1][y] == 1:
                        q.put((x - 1, y))
                    if y > 0 and a[x][y - 1] == 1:
                        q.put((x, y - 1))
                    if x < m - 1 and a[x + 1][y] == 1:
                        q.put((x + 1, y))
                    if y < n - 1 and a[x][y + 1] == 1:
                        q.put((x, y + 1))
                
    return res

print(get_number_of_islands([
    [1, 0, 1, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 1, 1, 0, 1],
]))

print(get_number_of_islands([
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
]))

print(get_number_of_islands([
    [],
]))
