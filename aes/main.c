// main.cpp

#include <stdio.h>
#include <stdint.h>

typedef uint8_t *Block_t;

#pragma region AES
void ShiftRows()
{
}

void InvShiftRows()
{
}

void MixColumns()
{
}

void InvMixColumns()
{
}

void AddRoundKey()
{
}

void SubBytes()
{
}

void InvSubBytes()
{
}

#pragma endregion /* AES */

int main(int argc, char **argv[])
{
    if (argc < 3)
        perror("Quantidade de argumentos insuficientes!");

    FILE *input_fp = fopen(argv[1], "r");

    if (input_fp == NULL)
        printf("Nao foi possivel abrir o arquivo!");
}