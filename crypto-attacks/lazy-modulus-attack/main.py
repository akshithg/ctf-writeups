from Crypto.Util.number import getPrime, inverse

p = getPrime(512)
q = getPrime(1024)
n = p * q
e = 65537
d = inverse(e, (p-1)*(q-1))

print(f"p: {p}")
print(f"q: {q}")
print(f"n: {n}")
print(f"e: {e}")
print(f"d: {d}")

# Attacker

def lazy_modulus_attack(n, p):
    q = n // p
    return p, q

p, q = lazy_modulus_attack(n, p)
print(f"Recovered p: {p}")
print(f"Recovered q: {q}")

assert p * q == n
