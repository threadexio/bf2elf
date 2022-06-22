all: debug

.ONESHELL:
test.asm:
	cargo run -- compile -o test.asm -s test test.bf

test.o: test.asm
	nasm -f elf64 test.asm

tester: test.o
	gcc tester.c test.o -o tester

debug: clean tester
	gdb ./tester

clean:
	rm -f test.asm test.o tester
