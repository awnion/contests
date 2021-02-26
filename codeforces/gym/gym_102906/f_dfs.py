import os
import sys
from io import BytesIO, IOBase
sys.setrecursionlimit(20_000)


#######################################################
#######################################################
class FastIO(IOBase):
    newlines = 0
    BUFSIZE = 1 << 13

    def __init__(self, file):
        self._fd = file.fileno()
        self.buffer = BytesIO()
        self.writable = "x" in file.mode or "r" not in file.mode
        self.write = self.buffer.write if self.writable else None

    def read(self):
        while True:
            b = os.read(self._fd, max(os.fstat(self._fd).st_size, self.BUFSIZE))
            if not b:
                break
            ptr = self.buffer.tell()
            self.buffer.seek(0, 2), self.buffer.write(b), self.buffer.seek(ptr)
        self.newlines = 0
        return self.buffer.read()

    def readline(self):
        while self.newlines == 0:
            b = os.read(self._fd, max(os.fstat(self._fd).st_size, self.BUFSIZE))
            self.newlines = b.count(b"\n") + (not b)
            ptr = self.buffer.tell()
            self.buffer.seek(0, 2), self.buffer.write(b), self.buffer.seek(ptr)
        self.newlines -= 1
        return self.buffer.readline()

    def flush(self):
        if self.writable:
            os.write(self._fd, self.buffer.getvalue())
            self.buffer.truncate(0), self.buffer.seek(0)


class IOWrapper(IOBase):
    def __init__(self, file):
        self.buffer = FastIO(file)
        self.flush = self.buffer.flush
        self.writable = self.buffer.writable
        self.write = lambda s: self.buffer.write(s.encode("ascii"))
        self.read = lambda: self.buffer.read().decode("ascii")
        self.readline = lambda: self.buffer.readline().decode("ascii")


sys.stdin, sys.stdout = IOWrapper(sys.stdin), IOWrapper(sys.stdout)
def input(): return sys.stdin.readline().rstrip("\r\n")
#####################################


def make_next_positions(a, n, k):
    positions = [n + 1] * ((n + 3)*k)
    for el in range(k):
        cur = n + 1
        for i in range(n - 1, -1, -1):
            if a[i] == el + 1:
                cur = i + 1
            positions[i*k + el] = cur
    return positions


class DP:
    __slots__ = 'k', 'n', 'm', 'a', 'b', 'next_a', 'next_b', 'cache', 'res', 'res_path'

    def __init__(self, k, n, m, a, b):
        self.k = k
        self.n = n
        self.m = m
        self.a = a
        self.b = b
        self.next_a = make_next_positions(a, n, k)
        self.next_b = make_next_positions(b, m, k)
        self.cache = [-1]*((m+3)*(n+3))
        self.res = -1
        self.res_path = []

    def dp(self, x, y):
        if x > self.n and y > self.m:
            return 0

        h = x*(self.m + 2) + y
        if self.cache[h] > -1:
            return self.cache[h]

        r = 5005
        for el in range(self.k):
            next_x = self.next_a[x*self.k + el]
            next_y = self.next_b[y*self.k + el]
            r = min(r, 1 + self.dp(next_x, next_y))

        self.cache[h] = r
        return r

    def backtrack(self, x, y, res):
        if res == 0:
            return
        else:
            for el in range(self.k):
                next_x = self.next_a[x*self.k + el]
                next_y = self.next_b[y*self.k + el]
                if res - 1 == self.dp(next_x, next_y):
                    self.res_path += el + 1,
                    self.backtrack(next_x, next_y, res - 1)
                    return


def main():
    k = int(input())
    n = int(input())
    *a, = map(int, input().split())
    m = int(input())
    *b, = map(int, input().split())
    n = len(a)
    m = len(b)

    dp = DP(k, n, m, a, b)
    r = dp.dp(0, 0)
    dp.backtrack(0, 0, r)

    print(r)
    print(*dp.res_path)


if __name__ == '__main__':
    main()
