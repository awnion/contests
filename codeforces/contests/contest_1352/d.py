T = int(input())


while T > 0:
    N = int(input())

    d = [int(x) for x in input().split()]
    l, r = 1, N - 1
    a, b = d[0], 0
    c = d[0] + 1
    t = 1

    while l <= r:
        
        if l <= r:
            s = 0
            while l <= r and s < c:
                s += d[r]
                b += d[r]
                r -= 1

            t += 1
            c = s + 1

        
        
        if l <= r:
            s = 0
            while l <= r and s < c:
                s += d[l]
                a += d[l]
                l += 1

            t += 1
            c = s + 1
        
    print(t, a, b)

    T -= 1
