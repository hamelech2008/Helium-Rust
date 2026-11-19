.global _open
.global _read
.global _write

.section .text
# SYSTEM CALLS
_read: # helium call: _read(fd, buffer, size)

        pushq %rbp
        movq %rsp, %rbp

        movq $0, %rax # sys read
        syscall # return value in rax

        movq %rbp, %rsp
        popq %rbp
        retq



_write: # helium call: _write(fd, buffer, size)
        pushq %rbp
        movq %rsp, %rbp
        
        movq $1, %rax # sys write
        syscall


        movq %rbp, %rsp
        popq %rbp
        retq

_open: # helium call: _open(filename, flags, mode)
        pushq %rbp
        movq %rsp, %rbp

        movq $2, %rax #sys open
        syscall # return value in rax

        movq %rbp, %rsp
        popq %rbp
        retq
