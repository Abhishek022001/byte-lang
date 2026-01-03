.global _main
.align 4
.text
_term:
stp x29, x30, [sp, #-16]!
mov x29, sp
mov x16, #1
svc #0x80
ldp x29, x30, [sp], #16
ret
_test:
stp x29, x30, [sp, #-16]!
mov x29, sp
ldp x29, x30, [sp], #16
ret
_main:
stp x29, x30, [sp, #-16]!
mov x29, sp
sub sp, sp, #48
mov x10, 5
str x10, [x29, #-44]
mov w10, 2
strh w10, [x29, #-42]
mov w10, 1
strb w10, [x29, #-41]
mov x10, 30
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
ldrsw x10, [x29, #-44]
str x10, [sp, #-12]
ldr x10, [x29, #-25]
str x10, [sp, #-4]
bl _test
add sp, sp, #16
sub sp, sp, #16
ldr x0, [x29, #-25]
ldrsw x10, [x29, #-44]
str x10, [sp, #-12]
ldrb w10, [x29, #-15]
strb w10, [sp, #-11]
bl _term
add sp, sp, #16
add sp, sp, #48
ldp x29, x30, [sp], #16
ret
