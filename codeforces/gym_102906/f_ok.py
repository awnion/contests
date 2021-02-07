import os
import sys
from io import BytesIO, IOBase
from collections import deque

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
#####################################

def pack(x, y):
    return (x + 2) * 5005 + y + 2

def unpack(h):
    x, y = divmod(h, 5005)
    return x - 2, y - 2

def main():
    k = int(input())
    n = int(input())
    *a, = map(int, input().split())
    m = int(input())
    *b, = map(int, input().split())
    n = len(a)
    m = len(b)

    visited = dict()

    q = deque()
    q.append(pack(-1, -1))

    while q:
        from_hash = q.popleft()
        from_x, from_y = unpack(from_hash)

        for el in range(1, k+1):
            x = min(from_x + 1, n)
            y = min(from_y + 1, m)

            while x < n and a[x] != el:
                x += 1
            while y < m and b[y] != el:
                y += 1

            if x == n and y == m:
                s = [el]
                x, y = from_x, from_y
                while x != -1 and y != -1:
                    s += (x < n) and a[x] or b[y],
                    x, y = unpack(visited[pack(x, y)])
                print(len(s))
                print(*s[::-1])
                return

            to_hash = pack(x, y)
            if to_hash not in visited:
                visited[to_hash] = from_hash
                q.append(to_hash)

if __name__ == '__main__':
    main()
