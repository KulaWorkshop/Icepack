#include <stdint.h>
#include <stdlib.h>
#include "lzrw.h"

typedef unsigned char BYTE;

BYTE* decompress(BYTE* buffer, size_t size, int* pSizeOut) {
    uint32_t sizeOut = 0;
    uint32_t workingMemoryLen = lzrw3_req_mem();
    BYTE* workingMemory = malloc(sizeof *workingMemory * workingMemoryLen);
    BYTE* bufferOut = malloc(sizeof *buffer * size * 10);
    lzrw3a_compress(COMPRESS_ACTION_DECOMPRESS, workingMemory, buffer, size, bufferOut, &sizeOut);
    *pSizeOut = sizeOut;

    free(workingMemory);
    return bufferOut;
}

BYTE* compress(BYTE* buffer, size_t size, int* pSizeOut) {
    uint32_t sizeOut;
    uint32_t workingMemoryLen = lzrw3_req_mem();
    BYTE* workingMemory = malloc(sizeof *workingMemory * workingMemoryLen);
    BYTE* bufferOut = malloc(sizeof *buffer * size * 10);
    lzrw3a_compress(COMPRESS_ACTION_COMPRESS, workingMemory, buffer, size, bufferOut, &sizeOut);
    *pSizeOut = sizeOut;

    free(workingMemory);
    return bufferOut;
}