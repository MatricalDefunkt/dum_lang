global _start

section .data
    pi dq 3.141
section .text
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, pi
    mov rdx, 8
    syscall

    mov rax, 60
    mov rdi, 0
    syscall