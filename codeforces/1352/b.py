T = int(input())


def even(x):
    return x % 2 == 0


while T > 0:
    N, K = map(int, input().split())

    ans = []

    a = N - (K - 1)
    b = N - 2 * (K - 1)

    if not even(a) and a > 0:
        print("YES")
        print(*[str(a)] + ["1"] * (K - 1))
    elif even(b) and b > 0:
        print("YES")
        print(*[str(b)] + ["2"] * (K - 1))
    else:
        print("NO")

    T -= 1
