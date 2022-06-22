#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "include/test.h"

typedef unsigned long long u64;

void* prepare_memory(u64 mem_size) {
	u64 size;
	if (mem_size < 30000) {
		size = 30000;
	} else {
		size = mem_size;
	}

	void* mem = malloc(size);
	memset(mem, 0, size);
	return mem;
}

int main() {
	u64	  pmem_size = 30000;
	void* pmem		= prepare_memory(pmem_size);

	void* dp = test(pmem);

	printf(" |        memory base  = %p\n", pmem);
	printf(" |        data pointer = %p\n", dp);

	if (pmem > dp || dp > pmem + pmem_size) {
		printf(
			" | Brainfuck code has errors! Data pointer went out of bounds!\n");
	} else {
		printf(" | data pointer offset = 0x%x\n", dp - pmem);
	}

	asm("int $3"); // trigger breakpoint

	free(pmem);
}
