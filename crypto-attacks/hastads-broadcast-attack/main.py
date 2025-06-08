from Crypto.Util.number import getPrime, inverse

p = getPrime(512)
q = getPrime(512)
n = p * q
e1 = 3
e2 = 5
e3 = 7
c1 = pow(m, e1, n)
c2 = pow(m, e2, n)
c3 = pow(m, e3, n)

print(f"p: {p}")
print(f"q: {q}")
print(f"n: {n}")
print(f"e1: {e1}")
print(f"e2: {e2}")
print(f"e3: {e3}")
print(f"c1: {c1}")
print(f"c2: {c2}")
print(f"c3: {c3}")

# Attacker

def chinese_remainder_theorem(M, N):
    result = 0
    prod = 1
    for n in N:
        prod *= n
    for n, m in zip(N, M):
        p = prod // n
        result += m * inverse(p, n) * p
    return result % prod

def hastad_broadcast_attack(c1, c2, c3, e1, e2, e3, n):
    N = [e1, e2, e3]
    C = [c1, c2, c3]
    M = [pow(c, e, n) for c, e in zip(C, N)]
    return chinese_remainder_theorem(M, N)

m = hastad_broadcast_attack(c1, c2, c3, e1, e2, e3, n)
print(f"Recovered m: {m}")
