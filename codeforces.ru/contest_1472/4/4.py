T=int(input())

def solve():
    n = int(input())
    a = [*map(int, input().split())]
    a.sort(reverse=True)
    A = 0
    B = 0
    i = 0
    while i < n:
        x = a[i]
        if i&1:
            B += (x&1) * x
        else:
            A += (x&1^1) * x
        i += 1
    if A > B:
        print('Alice')
    elif A < B:
        print('Bob')
    else:
        print('Tie')

while T:
    solve()
    T -= 1
