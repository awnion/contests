import sys
from time import time_ns

# For every direction: the 4 lines of cell indices, each listed from the side
# the tiles slide towards.
LINES = {
    'L': [[r * 4 + c for c in range(4)] for r in range(4)],
    'R': [[r * 4 + c for c in reversed(range(4))] for r in range(4)],
    'U': [[r * 4 + c for r in range(4)] for c in range(4)],
    'D': [[r * 4 + c for r in reversed(range(4))] for c in range(4)],
}
ROWS = LINES['L']
COLS = LINES['U']
# Free cells are counted column by column, top to bottom.
SPAWN_ORDER = [i for col in COLS for i in col]

SQUASHED = {}


def squash(line):
    res = SQUASHED.get(line)
    if res is not None:
        return res
    a = list(line)
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
    res = SQUASHED[line] = tuple(a)
    return res


def move(grid, direction, seed, value):
    a = list(grid)
    for i0, i1, i2, i3 in LINES[direction]:
        a[i0], a[i1], a[i2], a[i3] = squash((grid[i0], grid[i1], grid[i2], grid[i3]))

    free = [i for i in SPAWN_ORDER if a[i] == 0]
    if not free or tuple(a) == grid:
        return None

    a[free[seed % len(free)]] = value
    return tuple(a)


def argmax(line):
    return line.index(max(line))


def cmp(x):
    a, d = x
    nz = 16 - a.count(0)
    mx = sum(argmax([a[i] for i in col]) for col in COLS)
    my = sum(argmax(a[r:r + 4]) for r in (0, 4, 8, 12))
    pd = 12*(len(d) > 3 and d[-1] == d[-2] == d[-3])

    return nz + pd + mx + my*4


DEPTH = 11
MOVES = 8
WIDTH = 15

i = 0
while True:
    seed = int(input())
    score = int(input())

    a = tuple(int(v) for _ in range(4) for v in input().split())
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
            for direction in 'UDRL':
                g = move(prev_a, direction, predicted_seed, predicted_value)
                if g is not None:
                    q2.append((g, prev_d + direction))

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
