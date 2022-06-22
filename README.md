# bf2elf

- Ever had the dream of compiling brainfuck down to native machine code?
- Ever wanted to have a way to use brainfuck to achieve something?

If you answered yes to any of the previous questions, fear not, you are in the right place.

bf2elf is a compiler/interpreter for the brainfuck language. It allows you to test your code with the built-in interpreter and then compile it and link it with the rest of your project.

With this tool, brainfuck compiles down to *cough* relatively unoptimized *cough* x86_64 assembly which using `nasm` can be compiled down to an object file to link with other code.

## Usage

This compiler exports one function containing all the compiled brainfuck code. The name of this symbol is specified with the `-s/--symbol` parameter. For an example of how to call this function from C see `tester.c` and `include/test.h`

See the makefile for more.

## Notes

> You need to have `nasm` installed.

> Only 64-bit mode supported. No puny 32-bit computing here.

> Unlike the interpreter, the compiler does not support the `.` and `,` operators of the language, totally not because I don't want to have to deal with memory management in pure x86 assembly, but because I don't think it would be that useful, plus this kind of thing can be done before calling the brainfuck code through the memory block the caller has to allocate.
