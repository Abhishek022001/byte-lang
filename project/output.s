.global _main
.align 4
.text
_term:
stp x29, x30, [sp, #-16]!
mov x29, sp
sub sp, sp, #0
mov x16, #1
svc #0x80
add sp, sp, #0
ldp x29, x30, [sp], #16
ret
_main:
stp x29, x30, [sp, #-16]!
mov x29, sp
sub sp, sp, #16
mov w10, 5
str w10, [x29, #-12]
mov w10, 2
strh w10, [x29, #-10]
mov w10, 1
strb w10, [x29, #-9]
mov w10, 0
str w10, [x29, #-5]
ldr w10, [x29, #-5]
str w10, [x29, #-1]
sub sp, sp, #16
ldr x0, [x29, #-1]
ldr w10, [x29, #-12]
str w10, [sp, #-12]
bl _term
add sp, sp, #16
add sp, sp, #16
ldp x29, x30, [sp], #16
ret
