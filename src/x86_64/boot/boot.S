; Copyright 2016 The pliOS Developers. See the LICENSE
; file at the top-level directory of this distribution.
;
; Licensed under the MIT license <LICENSE-MIT or
; http://opensource.org/licenses/MIT>. This file may not
; be copied, modified, or distributed except according
; to these terms.

section .multiboot_header
header_start:
    dd 0xe85250d6
    dd 0
    dd header_end - header_start

    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))


    dw 0
    dw 0
    dd 8
header_end:

global start
extern start_64

section .early_text
bits 32
start:
    cli

    mov esp, stack_top
    mov edi, ebx

    call check_multiboot
    call check_cpuid
    call check_long_mode

    call set_up_page_tables
    call enable_paging

    lgdt [gdt64.pointer]

    jmp gdt64.code:start_64

; Check if multiboot is supported
check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "0"
    jmp error

; Check if CPUID is supported
check_cpuid:
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21
    push eax
    popfd
    pushfd
    pop eax
    push ecx
    popfd
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp error

; Check if long mode is supported
check_long_mode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode

    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error

; Setup page tables
set_up_page_tables:
    ; Link first P4 entry to P3
    mov eax, p3_table
    or eax, 0b11
    mov [p4_table], eax

    ; Link middle P4 entry to P3
    mov eax, p3_table
    or eax, 0b11
    mov [p4_table + 256*8], eax

    ; Link first, second, second to last and last P3 entries to P2 tables
    mov eax, p2_table
    or eax, 0b11
    mov [p3_table], eax

    mov eax, p2_table_2
    or eax, 0b11
    mov [p3_table + 1*8], eax

    mov ecx, 0

.map_p2_tables:
    mov eax, 0x200000
    mul ecx
    or eax, 0b10000011
    mov [p2_table + ecx * 8], eax

    inc ecx
    cmp ecx, 1024
    jne .map_p2_tables

    ret

enable_paging:
    ; load P4 to cr3 register (cpu uses this to access the P4 table)
    mov eax, p4_table
    mov cr3, eax

    ; enable PAE-flag in cr4 (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret

error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

section .early_rodata
gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
.pointer:
    dw $ - gdt64 - 1
    dq gdt64

section .early_data
align 4096
p4_table:
    times 512 dq 0
p3_table:
    times 512 dq 0
p2_table:
    times 512 dq 0
p2_table_2:
    times 512 dq 0
stack_bottom:
    times 64 db 0
stack_top:
