from bisect import bisect

def get_path(x, y, time):
    Time    = [[time, x]]
    TimeV   = {x:Time[0]}
    Visited = [False for k in range(0,N+1)]
    
    lenTime = 1
    
    while lenTime > 0:
        [minTime, minVertex] = Time.pop(0)
        lenTime -= 1
                
        if minVertex == y:
            Time.insert(0, [minTime, y])
            lenTime += 1
            break
        
        Visited[minVertex] = True
        
        if E.has_key(minVertex):
            for [B, C, D] in E[minVertex]:
                if not Visited[B]:
                    count = (minTime - 1) / C + 1 # checked
                    tmpTime = count * C + D
                    
                    if TimeV.has_key(B) and tmpTime < TimeV[B][0] or not TimeV.has_key(B):
                            l = bisect(Time, [tmpTime, B])
                            Time.insert(l, [tmpTime, B])
                            TimeV[B] = Time[l]
                            lenTime += 1
    
    if lenTime > 0 and Time[0][1] == y:
        return Time[0][0]
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