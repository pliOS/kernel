; Copyright 2016 The pliOS Developers. See the LICENSE
; file at the top-level directory of this distribution.
;
; Licensed under the MIT license <LICENSE-MIT or
; http://opensource.org/licenses/MIT>. This file may not
; be copied, modified, or distributed except according
; to these terms.


global start_64

section .text
bits 64
KERNEL_OFFSET equ 0xFFFF800000000000
start_64 equ _start_64 - KERNEL_OFFSET

_start_64:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov rax, .fixup_rip
    jmp rax

.fixup_rip:
    mov rsp, stack_top

    extern kmain_bp_arch
    call kmain_bp_arch

    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt

section .bss
stack_bottom:
    resb 4096
stack_top:
