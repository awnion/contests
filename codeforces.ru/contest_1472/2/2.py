T=int(input())

def solve():
    n = int(input())
    a = [*map(int, input().split())]
    c1 = a.count(1)
    c2 = a.count(2)
    s = sum(a)
    if s&1:
        print('NO')
    else:
        if c2&1 and c1 < 2 or c1&1:
            print('NO')
        else:
            print('YES')

while T:
    solve()
    T -= 1