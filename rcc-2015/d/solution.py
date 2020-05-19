MOD = 10**9 + 7
a, b, c = map(int, input().split())

p3 = lambda x: a*x*x + b*x + c

table = {}
for x in [p3(x) for x in range(10)]:
    table.setdefault(x, 0)
    table[x] += 1

print(table)

s = list(map(int, input().strip()))
print(s)

o = [0] * (len(s) + 1)
o[0] = 1


def solve(u, s, o, table, MOD):
    for i in range(u, len(s)):
        o[i + 1] = 0
        for j in range(3):
            # if i - j >= 0:
                try:
                    subs = 0
                    for k in s[i - j:i + 1]:
                        subs = subs * 10 + k

                    count = table[subs]
                    # print(subs, count, i, j)
                    o[i + 1] = (o[i + 1] + o[i - j] * table[subs]) % MOD
                except (KeyError, IndexError):
                    pass

    print(o)
    print(o[len(s)])

solve(0, s, o, table, MOD)
M = int(input())
for m in range(M):
    p, d = map(int, input().split())
    s[p - 1] = d
    # print(s)
    solve(max(0, int(p) - 1), s, o, table, MOD)
