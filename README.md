# Cryptography Algorithms in Rust

# Running

```
cargo run -- -Awarning
```

or 

```
./run.sh
```

# Testing and verifying results

You can use [CryptoTool Online - AES (step-by-step)](https://legacy.cryptool.org/en/cto/aes-step-by-step) to verify the results and understand each function implemented in this project.


Use the following parameters

- Number of Rounds: 11 (1 initial, 9 rounds, 1 final)
- Chaining: None

### Key:
```
2b7e1516 28aed2a6 abf71588 99cf4f3c
```

### S-Box:

Used as substitution matrix 

    637c777b f26b6fc5 3001672b fed7ab76 ca82c97d fa5947f0 add4a2af 9ca472c0 b7fd9326 363ff7cc 34a5e5f1 71d83115 04c723c3
    1896059a 071280e2 eb27b275 09832c1a 1b6e5aa0 523bd6b3 29e32f84 53d100ed 20fcb15b 6acbbe39 4a4c58cf d0efaafb 434d3385
    45f9027f 503c9fa8 51a3408f 929d38f5 bcb6da21 10fff3d2 cd0c13ec 5f974417 c4a77e3d 645d1973 60814fdc 222a9088 46eeb814 
    de5e0bdb e0323a0a 4906245c c2d3ac62 9195e479 e7c8376d 8dd54ea9 6c56f4ea 657aae08 ba78252e 1ca6b4c6 e8dd741f 4bbd8b8a 
    703eb566 4803f60e 613557b9 86c11d9e e1f89811 69d98e94 9b1e87e9 ce5528df 8ca1890d bfe64268 41992d0f b054bb16 


### MixColumns Matrix
This Matrix is does not need to be set on CrypTool, because its a fixed-matrix that is applyied on mix columns along the process

    02 03 01 01 
    01 02 03 01
    01 01 02 03
    03 01 01 02


### Input
    0000hello world!

As bytes

    3030303068656c6c6f20776f726c6421


# AES

- Cryptography over blocks


## Key Expansion

A 128-bits key is expanded to 11 keys of 128-bits each.


For each round a key is created based on the previous keys by rotating, substution and applying XOR over the keys.

## AddRoundKey - OK

### Inputs: 
- State Block
- 


## SubBytes - OK

## ShiftRows - Broken

## MixColumns - Can't test because shiftrows is broken

Following: https://crypto.stackexchange.com/questions/2402/how-to-solve-mixcolumns/95775#95775