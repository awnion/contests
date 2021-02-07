import os
import sys
from io import BytesIO, IOBase
from operator import itemgetter


def main():
    T = int(input())
    sys.stdout.write('\n'.join([solve() for _ in range(T)]))


def solve():
    n = int(input())
    res = [-1]*n
    sizes = [None]*n
    i = 0
    while i < n:
        a = [*map(int, input().split()), i]
        if a[0] > a[1]:
            a[0], a[1] = a[1], a[0]
        sizes[i] = a
        # w, h = map(int, input().split())
        # if w > h:
        #     sizes[i] = (h, w, i)
        # else:
        #     sizes[i] = (w, h, i)
        i += 1

    sizes.sort(key=itemgetter(0))
    # sizes.sort(key=lambda x:x[0])

    l = 0
    lw, lh, lj = sizes[l]
    min_h, min_h_j = lh, lj
    # skip first group and update best
    while l < n and lw == sizes[l][0]:
        lw, lh, lj = sizes[l]
        if lh < min_h:
            min_h, min_h_j = lh, lj
        l += 1
 
    r = l
 
    while r < n:
        lw, lh, lj = sizes[l]
        rh, rj = lh, lj
        # update right using best
        while r < n and sizes[r][0] == lw:
            _, rh, rj = sizes[r]
            if rh > min_h:
                res[rj] = min_h_j + 1
            r += 1
 
        if r == n: break
 
        # update best
        while l < r:
            lw, lh, lj = sizes[l]
            if lh < min_h:
                min_h, min_h_j = lh, lj
            l += 1

    return ' '.join([str(x) for x in res])


#######################################################
#######################################################
class FastIO(IOBase):
    newlines = 0
    BUFSIZE = 8192
 
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
        self.readline = lambda: self.buffer.readline()  #.decode("ascii")
 
 
sys.stdin, sys.stdout = IOWrapper(sys.stdin), IOWrapper(sys.stdout)
def input(): return sys.stdin.readline()  #.rstrip("\r\n")

if __name__ == '__main__':
    main()
