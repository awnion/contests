FOUT = open("d.in", "w")

print >> FOUT, 1

a = ''

for i in range(100000):
    a += '10000 '
    
print >> FOUT, a

FOUT.close()