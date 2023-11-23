#!/bin/bash

nasm -f elf64 out.asm
if [ $? -eq 0 ]; then
  ld out.o
  if [ $? -eq 0 ]; then
    ./a.out
    echo
    echo "Exit code:" $?
  fi

  rm out.o
  rm a.out
fi
