T = int(input())

for t in range(T):
    x = int(input())
    y = int(input())
    z = int(input())

    if x * y == z:
        print('Infinity')
    else:
        print('Finite')
