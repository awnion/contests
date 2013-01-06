FIN  = open("h.in",  "r")
FOUT = open("h.out", "w")

IN = FIN.read().split()

a, b = [int(IN.pop(0))], int(IN.pop(0))

i = 0

while b not in a and a != []:
    tmp = len(a)
    for j in range(tmp):
        t = a.pop(0)
        t1 = t + 3
        t2 = t * 4
        if t1 not in a and t1 <= b: a.append(t1)
        if t2 not in a and t2 <= b: a.append(t2)
    i += 1

if a == []:
    print >> FOUT, -1
else:
    print >> FOUT, i

FIN.close()
FOUT.close()