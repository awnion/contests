FIN  = open("c.in",  "r")
FOUT = open("c.out", "w")

IN = FIN.read().split()

etalon = list(IN.pop(0))
etalon.sort()

N = int(IN.pop(0))

for i in range(N):
    str = IN.pop(0)
    e = list(str)
    e.sort()
    if cmp(e, etalon) == 0:
        print >> FOUT, str

FIN.close()
FOUT.close()