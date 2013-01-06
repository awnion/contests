FIN  = open("d.in",  "r")
FOUT = open("d.out", "w")
IN = FIN.read().split()

IN.pop(0)
a = {}

for s in IN:
    i = int(s)
    if a.has_key(i):
        a[i] += 1
    else:
        a[i] = 1

x, y, first = 1, 1, True

for i in range(10000, 0, -1):
    if a.has_key(i):
        x += i * (a[i] / 2)
        y += i * (a[i] / 2)
        if a[i] % 2 != 0:
            if first: x += i
            else:     y += i
            first = not first
    
print >> FOUT, max([x, y])
print >> FOUT, min([x, y])

FIN.close()
FOUT.close()