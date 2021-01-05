T=int(input())

def solve():
    w, h, n = [*map(int, input().split())]
    c = 1
    while w and w%2 == 0:
        w //= 2
        c *= 2
    while h and h%2 == 0:
        h //= 2
        c *= 2
    if c >= n:
        print('YES')
    else:
        print('NO')


while T:
    solve()
    T -= 1
