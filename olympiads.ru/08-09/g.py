FIN  = open("g.in",  "r") ; FOUT = open("g.out", "w")
IN   = FIN.read().split()
import psyco
psyco.full()
N = int(IN.pop(0))
print >> FOUT, N*(N+1)/2
res = ''
for i in range(1, N+1):
    for j in range(N, i-1, -1): res += str(j) + ' '
res += '1'
print >> FOUT, res
FIN.close() ; FOUT.close()