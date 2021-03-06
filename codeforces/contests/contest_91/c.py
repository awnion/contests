cache = {}
def luck(n):
    try:
        return cache[n]
    except KeyError:
        pass
    if n == 1:
        return [4, 7]
    a = luck(n - 1)
    b = map(lambda x: x * 10 + 4, a)
    c = map(lambda x: x + 3, b)
    cache[n] = b + c
    return cache[n]


a, b = map(int, raw_input().split())
lucks = sorted(reduce(lambda x, y: x + y, [luck(i) for i in range(1,11)]))
lucks = [0] + lucks
summ = 0
for i in range(len(lucks) - 1):
    x, y = lucks[i], lucks[i + 1]
    if a <= x <= b:
        if a <= y <= b:
            summ += y * (y - x)
        if y > b:
            summ += y * (b - x)
    if x < a:
        if a <= y <= b:
            summ += y * (y - a + 1)
        if y > b:
            summ += y * (b - a + 1)

print summ

