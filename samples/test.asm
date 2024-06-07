format ELF64 executable 3
segment readable executable

entry _start__

add__:
    ;; Function Prolog
    push      rbp
    mov       rbp,      rsp

    ;; Add
    mov       rdx,      rdi
    add       rdx,      rsi

    mov       rax,      rdx

    ;; Function Epilog
    pop       rbp
    ret

addOne__:
    ;; Function Prolog
    push      rbp
    mov       rbp,      rsp

    ;; Call
    mov       rsi,      1
    call      add__

    ;; Function Epilog
    pop       rbp
    ret

main__:
    ;; Function Prolog
    push      rbp
    mov       rbp,      rsp

    ;; Call
    mov       rdi,      31
    call      addOne__

    ;; Function Epilog
    pop       rbp
    ret

_start__:
    call      main__

    ;; Prepare syscall arguments
    mov       rdi,      rax
    mov       rax,      60  ; syscall number for exit
    syscall

segment readable writable
