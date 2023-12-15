format ELF64 executable 3
segment readable executable

entry _start__
main__:
      push       rbp
       mov       rbp,       rsp
       mov       rdi,         1
       mov       rsi,         3
       mov       rdx,       rdi
       cmp       rdx,       rsi
      setg        al
     movzx       rdx,        al
      test       rdx,       rdx
        jz       .L0__
       mov       rcx,         4
       mov       rax,       rcx
       jmp     .exit__
.L0__:
       mov        r8,       100
       mov       rax,        r8
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