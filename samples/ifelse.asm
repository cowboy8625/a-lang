format ELF64 executable 3
segment readable executable

entry _start__
main__:
      push       rbp
       mov       rbp,       rsp
       mov       rdi,         2
       mov       rsi,         1
       mov       rdx,       rdi
       cmp       rdx,       rsi
      setg        al
     movzx       rdx,        al
      test       rdx,       rdx
        jz       .L0__
       mov       rcx,         1
       mov        r8,         4
       mov        r9,       rcx
       add        r9,        r8
       mov       rax,        r9
       jmp     .exit__
.L0__:
       mov       rcx,        20
       mov       rax,       rcx
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