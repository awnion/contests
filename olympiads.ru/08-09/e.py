from bisect import bisect

from time import time
start_time = time()

def get_path(x, y, time):
    Time    = {x:time}
    Visited = {x:False}
    
    while True:
        minTime, minVertex = 0, -1
        
        for v, t in Time.items():
            if not Visited[v] and (minVertex == -1 or t < minTime):
                minTime, minVertex = t, v
                
        if minVertex == -1 or minVertex == y:
            break
        
        Visited[minVertex] = True
        
        if E.has_key(minVertex):
            for [B, C, D] in E[minVertex]:
                count = (minTime - 1) / C + 1 # checked
                tmpTime = count * C + D
                    
                if Time.has_key(B):
                    if not Visited[B] and Time[B] > tmpTime:
                        Time[B] = tmpTime
                else:
                    Time[B], Visited[B] = [tmpTime, False]
    
    if Time.has_key(y):
        return Time[y]
    else:
        return -1

def get_int():
    return int(input.pop(0))

###############################################################################
# program                                                                     #
###############################################################################

import psyco
psyco.bind(get_path)

FIN  = open("e.in",  "r")
FOUT = open("e.out", "w")

input = FIN.read().split()

N = get_int()
K = get_int()
E = {}

for i in range(K):
    A, B, C, D = get_int(), get_int(), get_int(), get_int()
    if E.has_key(A):
        E[A].append([B,C,D])
    else:
        E[A] = [[B,C,D]]

M = get_int()

P1 = get_int()

result = 0

for i in range(1,M):
    P2 = get_int()
    result = get_path(P1, P2, result)
    if result == -1:
        break
    P1 = P2

print >> FOUT, result

FIN.close()
FOUT.close()

print time()-start_time