.global String_String
.global String_get
.global String_length

.section .text
String_length:
        pushq %rbp
        movq %rsp, %rbp
        pushq %rdi
        movq 0(%rsp), %rax
        pushq %rax
        popq %rax
        movq 8(%rax), %rbx
        pushq %rbx
        popq %rax
        movq %rbp, %rsp
        popq %rbp
        retq
        addq $8, %rsp
        movq %rbp, %rsp
        popq %rbp
        retq
String_String:
        pushq %rbp
        movq %rsp, %rbp
        pushq %rdi
        pushq %rsi
        pushq %rdx
        movq 0(%rsp), %rax
        pushq %rax
        popq %rax
        leaq 0(%rax), %rax
        pushq %rax
        movq 24(%rsp), %rax
        pushq %rax
        popq %rax
        popq %rbx
        movq %rax, (%rbx)
        movq 0(%rsp), %rax
        pushq %rax
        popq %rax
        leaq 8(%rax), %rax
        pushq %rax
        movq 16(%rsp), %rax
        pushq %rax
        popq %rax
        popq %rbx
        movq %rax, (%rbx)
        addq $0, %rsp
        movq %rbp, %rsp
        popq %rbp
        retq
String_get:
        pushq %rbp
        movq %rsp, %rbp
        pushq %rdi
        pushq %rsi
        movq 0(%rsp), %rax
        pushq %rax
        popq %rax
        movq 0(%rax), %rbx
        pushq %rbx
        movq 16(%rsp), %rax
        pushq %rax
        popq %rax
        popq %rbx
        movzbq (%rbx, %rax, 1), %rax
        pushq %rax
        popq %rax
        movq %rbp, %rsp
        popq %rbp
        retq
        addq $8, %rsp
        movq %rbp, %rsp
        popq %rbp
        retq
