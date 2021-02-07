T = int(input())


while T > 0:
    N, K = map(int, input().split())

    r = K % (N - 1)
    a = K // (N - 1)

    print(a * N + r - (0 if r > 0 else 1))

    T -= 1
