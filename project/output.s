.global _main
.align 4
.text
_main:
stp x29, x30, [sp, #-16]!
mov x29, sp
sub sp, sp, #48
mov w10, 5
str w10, [x29, #-44]
mov w10, 2
strh w10, [x29, #-42]
mov w10, 1
strb w10, [x29, #-41]
mov x10, 0
str x10, [x29, #-33]
ldr x10, [x29, #-33]
str x10, [x29, #-25]
mov x10, 10
str x10, [x29, #-17]
mov w10, 30
strb w10, [x29, #-16]
ldrb w10, [x29, #-16]
strb w10, [x29, #-15]
sub sp, sp, #16
ldr x0, [x29, #-25]
ldrsw x10, [x29, #-44]
str w10, [sp, #-12]
ldrb w10, [x29, #-15]
strb w10, [sp, #-11]
bl _term
add sp, sp, #16
add sp, sp, #48
ldp x29, x30, [sp], #16
ret
_term:
stp x29, x30, [sp, #-16]!
mov x29, sp
sub sp, sp, #0
mov x1, 1
mov x16, #1
svc #0x80
add sp, sp, #0
ldp x29, x30, [sp], #16
ret
