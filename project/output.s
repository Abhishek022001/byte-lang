.global _main
.align 4
.text
_term:
stp x29, x30, [sp, #-16]!
mov x29, sp
sub sp, sp, #0

mov x0, #1
mov x16, #1
svc #0x80
add sp, sp, #0
ldp x29, x30, [sp], #16
ret
_main:
stp x29, x30, [sp, #-16]!
mov x29, sp
sub sp, sp, #16
mov x10, #0
str x10, [x29, #0]
bl _term
add sp, sp, #16
ldp x29, x30, [sp], #16
ret
