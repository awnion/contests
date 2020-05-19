mod = 10 ** 9 + 7

r = 0
o = 0
q = 500000004

# s = open(0, 'rb').read()

# for c in s:
#     if c == 63:
#         r <<= 1
#         r += o * q
#         r %= mod

#         q <<= 1
#         q %= mod

#         o += 1
#     elif c == 48:
#         r += o * q
#         r %= mod
#     else:
#         o += 2

# f1 = lambda x: (x << 1) % p
# f2 = lambda x: (x + o * q) % p

f1 = lambda x, p: (x << 1) % p
f2 = lambda x, o, q, p: (x + o * q) % p

for c in open(0, 'rb').read():

        if c == 63:
            r = f1(r, mod)
            r = f2(r, o, q, mod)

            q = f1(q, mod)

            o += 1
        elif c == 48:
            r = f2(r, o, q, mod)
        else:
            o += 2

print(r)
