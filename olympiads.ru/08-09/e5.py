from bisect import bisect

def get_path(x, y, time):

    Time = [[time, x]]
    TimeV = {x:Time[0]}
    Visited = [False for k in xrange(N+1)]
    
    while len(Time) > 0:
        [minTime, minVertex] = Time.pop(0)
        
        if minVertex == y:
            break
        
        Visited[minVertex] = True
        
        try:
            for [B, C, D] in E[minVertex]:
                count = (minTime - 1) / C + 1 # checked
                tmpTime = count * C + D
                
                try:
                    if tmpTime < TimeV[B][0]:
                        l = bisect(Time, [tmpTime, B])
                        Time.insert(l, [tmpTime, B])
                        TimeV[B] = Time[l]
                except:
                    l = bisect(Time, [tmpTime, B])
                    Time.insert(l, [tmpTime, B])
                    TimeV[B] = Time[l]
        except:
            pass
                
    if minVertex == y:
        return minTime
    else:
        return -1

###############################################################################
# program                                                                     #
###############################################################################

import psyco
psyco.full()

FIN  = open("e.in",  "r")
FOUT = open("e_right.out", "w")

input = FIN.readline().split()

N = int(input[0])
K = int(input[1])
E = {}

for i in xrange(K):
    input = FIN.readline().split()
    A, B, C, D = int(input[0]), int(input[1]), int(input[2]), int(input[3])
    try:
        E[A] += [[B,C,D]]
    except:
        E[A] = [[B,C,D]]

input = FIN.read().split()

M = int(input[0])

P1 = int(input[1])

result = 0

for i in xrange(1,M):
    P2 = int(input[i+1])
    result = get_path(P1, P2, result)
    if result == -1:
        break
    P1 = P2

print >> FOUT, result

FIN.close()
FOUT.close()