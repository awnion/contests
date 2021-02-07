T=int(input())

def solve():
    n = int(input())
    a = [*map(int, input().split())]
    b = [0] * n
    i = n - 1
    while i >= 0:
        x = a[i]
        if i + x >= n:
            b[i] = x
        else:
            b[i] = b[i + x] + x
        i -= 1

    print(max(b))


while T:
    solve()
    T -= 1