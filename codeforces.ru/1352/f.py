T = int(input())


while T > 0:
    a, b, c = [int(x) for x in input().split()]

    x = '0' * a
    y = '1' * c

    if b == 0:
        if a > 0:
            print(x + '0')
        else:
            print(y + '1')
    elif b % 2 == 1:
        print(x + '01' * ((b + 1) // 2) + y)
    else:
        print(x + '01' + y + '0' + '10' * ((b - 1) // 2))
    
    T -= 1
