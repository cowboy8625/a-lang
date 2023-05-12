format ELF64 executable 3
segment readable executable

entry _start__
max__:
      push       rbp
       mov       rbp,       rsp
       mov       rdi,       rsi
       cmp       rdi,       rdx
      setg        al
     movzx       rdi,        al
      test       rdi,       rdi
        jz       .L0__
       mov       rax,       rsi
       jmp     .exit__
.L0__:
       mov       rax,       rdx
       jmp     .exit__
.exit__:
       mov       rbp,       rsp
       pop       rbp
       ret
main__:
      push       rbp
       mov       rbp,       rsp
       mov       rdi,         1
       mov       rsi,         2
      call       max__
       mov       rax,       rdx
       jmp     .exit__
.exit__:
       mov       rbp,       rsp
       pop       rbp
       ret


_start__:
      call      main__
       mov       rdi,       rax
       mov       rax,        60
   syscall

segment readable writable