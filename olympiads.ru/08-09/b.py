#!/usr/bin/python

FIN  = open("b.in",  "r")
FOUT = open("b.out", "w")

InputData = FIN.read().split()

VasiaTime = int(InputData.pop(0))*60 + int(InputData.pop(0))

N = int(InputData.pop(0))

array = []

for i in range(N):
    T = int(InputData.pop(0))
    M = int(InputData.pop(0))
    for j in range(M):
        array.append((int(InputData.pop(0))*60 + int(InputData.pop(0)) + T)%1440)
    
for i in range(len(array)):
    array[i] -= VasiaTime
    if array[i] < 0: array[i] += 1440
    
Result = min(array)

print >> FOUT, Result

FIN.close()
FOUT.close()
