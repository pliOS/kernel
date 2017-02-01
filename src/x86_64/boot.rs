// Copyright 2016 The pliOS Developers. See the LICENSE
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>. This file may not
// be copied, modified, or distributed except according
// to these terms.

use core::cmp;
use multiboot2;
use spin::Mutex;
use core::mem::size_of;

#[macro_use]
use arch;

use utils;
use pfa;

pub static BOOT_INFO: Mutex<BootInfo> = Mutex::new(BootInfo {
    multiboot_addr: 0,
    kernel_start: 0,
    kernel_end: 0,
    multiboot_start: 0,
    multiboot_end: 0,
});

pub struct BootInfo {
    /// The virtual address of the multiboot header
    pub multiboot_addr: usize,
    /// The kernel start address (physical)
    pub kernel_start: usize,
    /// The kernel end address (physical)
    pub kernel_end: usize,
    /// The Multiboot boot information start address (physical)
    pub multiboot_start: usize,
    /// The Multiboot boot information end address (physical)
    pub multiboot_end: usize,
}

#[no_mangle]
pub extern fn kmain_bp_arch(multiboot_ebx: usize) {
    let mut boot_info = BOOT_INFO.lock();

    arch::early_printk::init();

    early_println!("pliOS booting up");

    boot_info.multiboot_addr = multiboot_ebx + arch::KERNEL_OFFSET;

    let mb_info = unsafe { multiboot2::load(boot_info.multiboot_addr) };
    let memory_map = mb_info.memory_map_tag().expect("Memory map required");
    let elf_sections = mb_info.elf_sections_tag().expect("ELF sections required");

    boot_info.kernel_start = elf_sections.sections().map(|s| elf_physical(s, false)).min().unwrap();
    boot_info.kernel_end   = elf_sections.sections().map(|s| elf_physical(s, true)).max().unwrap();

    boot_info.multiboot_start = multiboot_ebx;
    boot_info.multiboot_end   = multiboot_ebx + (mb_info.total_size as usize);

    let mem_length: usize = memory_map.memory_areas().map(|area| area.length as usize).sum();

    early_println!("{} MB of memory detected.", mem_length/(1024*1024));

    let bump_start_u = cmp::max(boot_info.kernel_end, boot_info.multiboot_end);
    let bump_start   = utils::round_up_2(bump_start_u, 0x1000);

    pfa::init(bump_start, mem_length);

    for area in memory_map.memory_areas() {
        pfa::add_memory_region(area.base_addr as usize, area.length as usize);
    }

    loop{}
}

fn elf_physical(s: &multiboot2::ElfSection, end: bool) -> usize {
    let addr = s.addr as usize;

    let phys_addr = if addr > arch::KERNEL_OFFSET {
        (addr - arch::KERNEL_OFFSET)
    } else {
        addr
    };

    if end {
        phys_addr + (s.size as usize)
    } else {
        phys_addr
    }
}
