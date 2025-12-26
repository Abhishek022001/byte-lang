.global _start
.align 2
.text
_start:

stp x29, x30, [sp, #-16]!
mov x29, sp
sub sp, sp, #16
mov x10, #60
str x10, [x29, #0]
