.global File_File
.global File_read
.global File_write

.section .text
# Files I/O
File_write:
        pushq %rbp
        movq %rsp, %rbp
        pushq %rdi
        pushq %rsi
        movq 0(%rsp), %rax
        pushq %rax
        popq %rax
        movq 0(%rax), %rbx
        pushq %rbx
        popq %rdi
        movq 8(%rsp), %rax
        pushq %rax
        popq %rax
        movq 0(%rax), %rbx
        pushq %rbx
        popq %rsi
        movq 8(%rsp), %rax
        pushq %rax
        popq %rdx
        call _write
        pushq %rax
        popq %rax
        movq %rbp, %rsp
        popq %rbp
        retq
        addq $8, %rsp
        movq %rbp, %rsp
        popq %rbp
        retq
File_read:
        pushq %rbp
        movq %rsp, %rbp
        pushq %rdi
        pushq %rsi
        movq 8(%rsp), %rax
        pushq %rax
        popq %rdi
        call malloc
        pushq %rax
        movq 8(%rsp), %rax
        pushq %rax
        popq %rax
        movq 0(%rax), %rbx
        pushq %rbx
        popq %rdi
        movq 0(%rsp), %rax
        pushq %rax
        popq %rsi
        movq 16(%rsp), %rax
        pushq %rax
        popq %rdx
        call _read
        movq $16, %rdi
        call malloc
        pushq %rax
        movq 8(%rsp), %rax
        pushq %rax
        popq %rdi
        movq 24(%rsp), %rax
        pushq %rax
        popq %rsi
        movq 0(%rsp), %rdx
        call String_String
        popq %rax
        movq %rbp, %rsp
        popq %rbp
        retq
        addq $0, %rsp
        movq %rbp, %rsp
        popq %rbp
        retq
File_File:
        pushq %rbp
        movq %rsp, %rbp
        pushq %rdi
        pushq %rsi
        movq 0(%rsp), %rax
        pushq %rax
        popq %rax
        leaq 0(%rax), %rax
        pushq %rax
        movq 16(%rsp), %rax
        pushq %rax
        popq %rax
        movq 0(%rax), %rbx
        pushq %rbx
        popq %rdi
        movq $2, %rax
        pushq %rax
        popq %rsi
        movq $420, %rax
        pushq %rax
        popq %rdx
        call _open
        pushq %rax
        popq %rax
        popq %rbx
        movq %rax, (%rbx)
        addq $0, %rsp
        movq %rbp, %rsp
        popq %rbp
        retq

