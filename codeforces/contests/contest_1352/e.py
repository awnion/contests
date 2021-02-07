import io, os
from itertools import accumulate, islice


def solve(a):
    N = len(a)
    aa = [0] + list(accumulate(a))
    d = [0] * 8001
    for i in range(N):
        for j in range(i + 2, N + 1):
            cur = aa[j] - aa[i]
            if cur > N:
                break
            d[cur] = 1

    print(sum(d[x] for x in a))


for s in islice(io.BytesIO(os.read(0, os.fstat(0).st_size)).readlines(), 2, None, 2):
    solve([int(x) for x in s.split()])
