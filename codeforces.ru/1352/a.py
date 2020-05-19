T = int(input())

while T > 0:
    N = int(input())

    ans = []
    tens = 1
    while N > 0:

        r = N % 10

        if r != 0:
            ans.append(f'{tens * r}')

        N //= 10
        tens *= 10

    print(len(ans))
    print(' '.join(ans))

    T -= 1
