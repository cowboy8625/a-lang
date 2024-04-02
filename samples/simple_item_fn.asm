format ELF64 executable 3
segment readable executable

entry _start__
add__:
    ;; Enter
    push       rbp
    mov       rbp,       rsp
    ;; Add
    mov       rdx,       rdi
    add       rdx,       rsi
    ;; Return
    mov       rax,       rdx
    ;; Jump
    jmp     .exit__
    ;; DefLabel
.exit__:
    ;; Leave
    mov       rbp,       rsp
    pop       rbp
    ret
addOne__:
    ;; Enter
    push       rbp
    mov       rbp,       rsp
    mov       rsi,         1
    ;; Call
    call       add__
    ;; Jump
    jmp     .exit__
    ;; DefLabel
.exit__:
    ;; Leave
    mov       rbp,       rsp
    pop       rbp
    ret
main__:
    ;; Enter
    push       rbp
    mov       rbp,       rsp
    mov       rdi,        31
    ;; Call
    call    addOne__
    ;; Jump
    jmp     .exit__
    ;; DefLabel
.exit__:
    ;; Leave
    mov       rbp,       rsp
    pop       rbp
    ret


_start__:
    call      main__
    mov       rdi,       rax
    mov       rax,        60
    syscall

segment readable writable
