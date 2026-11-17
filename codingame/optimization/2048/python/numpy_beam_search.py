import sys
import math
from time import time_ns
import numpy as np

DX = map(int, '0 0 1 -1'.split())
DY = map(int, '-1 1 0 0'.split())
MOVE = tuple('UDRL')


def squash(a):
    n = len(a)
    w, r = 0, 0
    while w < n and r < n:
        if r <= w:
            r = w + 1
        while r < n and a[r] == 0:
            r += 1
        if r >= n:
            break
        if a[w] == 0:
            a[w] = a[r]
            a[r] = 0
            r += 1
        else:
            if a[w] == a[r]:
                a[w] += a[r]
                a[r] = 0
            w += 1
    return a


def move(grid, direction, seed, value):
    k = "LURD".index(direction)
    a = np.copy(grid)
    b = np.rot90(a, k)
    for i in range(4):
        b[i] = squash(b[i])

    if (a == grid).all() or np.count_nonzero(a) == 16:
        return None

    x, y = np.nonzero(a.T == 0)
    pos = seed % x.size
    a[y[pos], x[pos]] = value
    # c = 0
    # for i in range(16):
    #     if a[i % 4][i // 4] == 0:
    #         if c == pos:
    #             a[i % 4][i // 4] = value
    #             break
    #         c += 1

    # x, y = free[seed % len(free)]
    # a[x][y] = value

    return a


def cmp(x):
    a, d = x
    nz = np.count_nonzero(a)
    mx = np.argmax(a, axis=0).sum()
    my = np.argmax(a, axis=1).sum()
    pd = 12*(len(d) > 3 and d[-1] == d[-2] == d[-3])

    return nz + pd + mx + my*4


DEPTH = 11
MOVES = 8
WIDTH = 15

i = 0
while True:
    seed = int(input())
    score = int(input())

    data = ';'.join([input(), input(), input(), input()])
    a = np.array(np.mat(data), int)
    ts = time_ns()

    predicted_seed = seed
    q = [(a, '')]
    for i in range(DEPTH):
        if (time_ns() - ts) // 1000_000 > 22:
            tf = time_ns()
            print('br', (tf - ts)//(10**6), file=sys.stderr)
            break
        q2 = []
        predicted_value = 2 + 2 * bool(predicted_seed & 0x10)
        for prev_a, prev_d in q:
            g = move(prev_a, 'U', predicted_seed, predicted_value)
            if g is not None:
                q2.append((g, prev_d + 'U'))
            g = move(prev_a, 'D', predicted_seed, predicted_value)
            if g is not None:
                q2.append((g, prev_d + 'D'))
            g = move(prev_a, 'R', predicted_seed, predicted_value)
            if g is not None:
                q2.append((g, prev_d + 'R'))
            g = move(prev_a, 'L', predicted_seed, predicted_value)
            if g is not None:
                q2.append((g, prev_d + 'L'))

        q = sorted(q2, key=cmp)[:WIDTH]
        predicted_seed = predicted_seed * predicted_seed % 50515093

    q.sort(key=cmp)

    tf = time_ns()
    print((tf - ts)//(10**6), file=sys.stderr)
    if q:
        print(q[0][1][:MOVES])
    else:
        print('RULD'[i % 4])
    i += 1
