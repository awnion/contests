FOUT = open("e.in",  "w")

print >> FOUT, 10000, 20000

for i in range(1, 10001):
    print >> FOUT, "%i %i 1 1" % (i/10000+1, i%10000 + 1)
for i in range(1, 10001):
    print >> FOUT, "%i %i 1 1" % (i%10000+1, i/10000 + 1)
    
print >> FOUT, 50
for i in range(1, 51):
    print >> FOUT, i * 200
    
FOUT.close()
    