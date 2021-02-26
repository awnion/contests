import os
import sys
from io import BytesIO, IOBase


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
    positions = [n + 1] * ((n + 2)*(k + 1))
    for el in range(k):
        cur = n + 1
        for i in range(n - 1, -1, -1):
            if a[i] == el + 1:
                cur = i + 1
            positions[i*k + el] = cur
    return positions


def main():
    k = int(input())
    n = int(input())
    *a, = map(int, input().split())
    m = int(input())
    *b, = map(int, input().split())
    n = len(a)
    m = len(b)

    next_pos_a = make_next_positions(a, n, k)
    next_pos_b = make_next_positions(b, m, k)

    n += 1
    m += 1

    best_next_a = [-1]*(m + 1)
    best_next_a[0] = 0

    all_prev_pos = []
    all_prev_el = []

    it = 0
    while best_next_a[m] < n:
        new_best_next_a = [-1]*(m + 1)
        prev_pos = [0]*(m + 1)
        prev_el = [0]*(m + 1)
        all_prev_pos.append(prev_pos)
        all_prev_el.append(prev_el)

        for i in range(m, -1, -1):
            if best_next_a[i] > -1:
                for el in range(k):
                    next_a = next_pos_a[best_next_a[i]*k + el]
                    next_b = next_pos_b[i*k + el]

                    if next_a > new_best_next_a[next_b]:
                        new_best_next_a[next_b] = next_a
                        prev_pos[next_b] = i
                        prev_el[next_b] = el

        best_next_a = new_best_next_a
        it += 1

    s = []
    j = m
    while it > 0:
        it -= 1
        s += [all_prev_el[it][j] + 1]
        j = all_prev_pos[it][j]

    print(len(s))
    print(*s[::-1])
    return


if __name__ == '__main__':
    main()
