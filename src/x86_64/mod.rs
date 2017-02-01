// Copyright 2016 The pliOS Developers. See the LICENSE
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>. This file may not
// be copied, modified, or distributed except according
// to these terms.

#[macro_use]
pub mod early_printk;

pub mod boot;
pub mod panic;

pub const KERNEL_OFFSET: usize = 0xffff800000000000;
pub const PAGE_SIZE: usize = 0x1000;
