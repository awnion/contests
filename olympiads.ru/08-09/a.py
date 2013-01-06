#!/usr/bin/python

def GetAnswer(a, b):
    if a == 21: return 2
    if b == 21: return 3
    return ((a + b) / 5) % 2 != 0

FIN  = open("a.in",  "r")
FOUT = open("a.out", "w")

InputData = FIN.read().split()

Answers = ["Vasya serves", "Petya serves", "Vasya wins", "Petya wins"]

print >> FOUT, Answers[GetAnswer(int(InputData[0]), int(InputData[1]))]

FIN.close()
FOUT.close()
