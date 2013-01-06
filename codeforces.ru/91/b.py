def luck(k):
    for i in k:
        if i not in ("4", "7"):
            return False
    return True

a = raw_input()
l = len(a)

d = {}

for i in range(l):
    for j in range(i + 1, l + 1):
        try:
            d[a[i:j]] += 1
        except KeyError:
            d[a[i:j]] = 1

m = 0
p = ""

for k in sorted(d.keys()):
    if luck(k) and d[k] > m:
        p = k
        m = d[k]

if p == "":
    print -1
else:
    print p
