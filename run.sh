#!/bin/bash

nasm -f elf64 out.asm;
ld out.o;
./a.out;
echo $?;

rm out.o;
rm a.out;