T = int(input())

for t in range(T):
    n, l = map(int, input().split())
    a = sorted(map(int, input().split()))
    b = sorted(map(int, input().split()))

    if sum(a[:l]) > sum(b[-l:]):
        print('YES')
    else:
        print('NO')
