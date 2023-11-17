
        global _start
        section .text
        section .data

                                    abc: dd 1
                                
        _start:
        
                    mov rax, 60
                    mov rdi, 1
                    syscall
                
        