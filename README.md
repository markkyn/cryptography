# Cryptography Algorithms in Rust


# AES

- Cryptography over blocks


## Key Expansion

A 128-bits key is expanded to 11 keys of 128-bits each.


For each round a key is created based on the previous keys by rotating, substution and applying XOR over the keys.

## AddRoundKey

## SubBytes

## ShiftRows

## MixColumns

Following: https://crypto.stackexchange.com/questions/2402/how-to-solve-mixcolumns/95775#95775