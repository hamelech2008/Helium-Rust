.global print
.global input
.global println


.section .text
# System I/O
print:
        pushq %rbp
        movq %rsp, %rbp
        pushq %rdi
        movq $1, %rax
        pushq %rax
        popq %rdi
        movq 0(%rsp), %rax
        pushq %rax
        popq %rax
        movq 0(%rax), %rbx
        pushq %rbx
        popq %rsi
        movq 0(%rsp), %rax
        pushq %rax
        popq %rax
        movq 8(%rax), %rbx
        pushq %rbx
        popq %rdx
        call _write
        addq $0, %rsp
        movq %rbp, %rsp
        popq %rbp
        retq


println:
	pushq %rbp
	movq %rsp, %rbp
	pushq %rdi
	movq 0(%rsp), %rax
	pushq %rax
	popq %rdi
	call print
	movq $2, %rdi
	call malloc
	pushq %rax
	movq $10, %rbx
	movb %bl, 0(%rax)
	movq $16, %rdi
	call malloc
	popq %rdi
	pushq %rax
	movq $2, %rsi
	movq %rax, %rdx
	call String_String
	popq %rdi
	call print
	addq $0, %rsp
	movq %rbp, %rsp
	popq %rbp
	retq


input:
        pushq %rbp
        movq %rsp, %rbp
        movq $50, %rax
        pushq %rax
        movq 0(%rsp), %rax
        pushq %rax
        popq %rdi
        call malloc
        pushq %rax
        movq $0, %rax
        pushq %rax
        popq %rdi
        movq 0(%rsp), %rax
        pushq %rax
        popq %rsi
        movq 8(%rsp), %rax
        pushq %rax
        popq %rdx
        call _read
        pushq %rax
        movq $16, %rdi
        call malloc
        pushq %rax
        movq 16(%rsp), %rax
        pushq %rax
        popq %rdi
        movq 8(%rsp), %rax
        pushq %rax
        popq %rsi
        movq 0(%rsp), %rdx
        call String_String
        popq %rax
        movq %rbp, %rsp
        popq %rbp
        retq
        addq $16, %rsp
        movq %rbp, %rsp
        popq %rbp
        retq
