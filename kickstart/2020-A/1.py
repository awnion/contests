t = int(input())

for case in range(1, t + 1):
    n, b = [int(x) for x in input().split()]
    arr = [int(x) for x in input().split()]
    a = [0] * 1001
    for x in arr:
        a[x] += 1

    j = 0
    s = 0

    for i, x in enumerate(a):
        while x and s + i <= b:
            s += i
            x -= 1
            j += 1


    print('Case #{}: {}'.format(case, j))
