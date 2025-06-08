from Crypto.Cipher import AES
from Crypto.Random import get_random_bytes
from Crypto.Util.Padding import pad, unpad

BLOCK_SIZE = 16

# Victim setup
key = get_random_bytes(BLOCK_SIZE)

def encrypt(plaintext: bytes) -> tuple[bytes, bytes]:
    iv = get_random_bytes(BLOCK_SIZE)
    cipher = AES.new(key, AES.MODE_CBC, iv)
    ciphertext = cipher.encrypt(pad(plaintext, BLOCK_SIZE))
    return iv, ciphertext

def padding_oracle(iv: bytes, ciphertext: bytes) -> bool:
    try:
        cipher = AES.new(key, AES.MODE_CBC, iv)
        unpad(cipher.decrypt(ciphertext), BLOCK_SIZE)
        return True
    except ValueError:
        return False

# Attacker
def decrypt_block(prev_block: bytes, target_block: bytes) -> bytes:
    recovered = bytearray(BLOCK_SIZE)
    intermediate = bytearray(BLOCK_SIZE)
    for i in range(1, BLOCK_SIZE + 1):
        pad_byte = i
        for guess in range(256):
            prefix = bytearray(BLOCK_SIZE)
            for j in range(1, i):
                prefix[-j] = intermediate[-j] ^ pad_byte
            prefix[-i] = guess
            if padding_oracle(bytes(prefix), target_block):
                intermediate[-i] = guess ^ pad_byte
                recovered[-i] = intermediate[-i] ^ prev_block[-i]
                break
    return bytes(recovered)

def full_decrypt(iv: bytes, ciphertext: bytes) -> bytes:
    blocks = [iv] + [ciphertext[i:i+BLOCK_SIZE] for i in range(0, len(ciphertext), BLOCK_SIZE)]
    plaintext = b""
    for i in range(1, len(blocks)):
        decrypted = decrypt_block(blocks[i-1], blocks[i])
        plaintext += decrypted
    return unpad(plaintext, BLOCK_SIZE)

# Simulate attack
plaintext = b"Top secret message, please don't leak me."
iv, ciphertext = encrypt(plaintext)
recovered_plaintext = full_decrypt(iv, ciphertext)

print(f"Original plaintext : {plaintext}")
print(f"Recovered plaintext: {recovered_plaintext}")
