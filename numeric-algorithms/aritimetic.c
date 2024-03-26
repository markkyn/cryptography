#include <stdio.h>
#include <stdint.h>

typedef struct aritimetic
{
    uint8_t *bytes;
    uint32_t block_len;

} Block_t;

void createBlock(){}

Block_t XOR(Block_t A, Block_t B)
{
    Block_t R;

    for(int b = 0; b < A.block_len; b++)
        A.bytes[b] ^= B.bytes[b];

    return 
}

Block_t Addition(Block_t A, Block_t B)
{
    for (int b = 0; b < A.block_len; b++)
        XOR(A, B)
}

Block_t Subtraction(Block_t A, Block_t B)
{
}

Block_t Multiplication(Block_t A, Block_t B)
{
}

int main(int argc, char **argv[])
{



}