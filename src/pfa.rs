// Copyright 2016 The pliOS Developers. See the LICENSE
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>. This file may not
// be copied, modified, or distributed except according
// to these terms.

//! Page frame allocator

use spin::Mutex;
use core::ptr::Unique;
use core::mem::size_of;

#[macro_use]
use arch;

use utils;

/// Initialize the allocator
///
/// bump_start is where the page entries are allocated from
/// length is the total length of the memory in the system
pub fn init(bump_start: usize, length: usize) {
    ALLOCATOR.lock().init(bump_start, length);
}

/// Add a memory region to this allocator.
///
/// from is the start address of the region
/// length is the number of pages in this region
pub fn add_memory_region(from: usize, length: usize) {
    ALLOCATOR.lock().add_memory_region(from, length);
}

static ALLOCATOR: Mutex<BuddyAlloc> = Mutex::new(BuddyAlloc::new());

pub struct Page {
    /// Reference count
    use_count: u32,
    /// Order (how many pages were allocated with this one)
    order: u8,
    /// Page frame number (address divided by page size)
    page_number: usize,
    /// Next page
    next: Option<Unique<Page>>,
    /// Next free page
    next_free: Option<Unique<Page>>,
}

struct BuddyMap {
    /// List of free pages
    pages: Option<Unique<Page>>,
    /// Length of the pages list (multiply by 2**order to get number of pages)
    length: usize,
}

struct BuddyAlloc {
    /// List of all pages
    pages: Option<Unique<Page>>,
    /// Length of the pages list
    length: usize,
    /// Buddy maps
    buddies: [BuddyMap; 6],
    /// Allocator info bump allocator start address
    bump_start: usize,
    /// Allocator info bump allocator current address
    bump_current: usize,
    /// Allocator info bump allocator end address
    bump_end: usize,
}

impl BuddyAlloc {
    const fn new() -> BuddyAlloc {
        BuddyAlloc {
            pages: None,
            length: 0,
            buddies: [
                BuddyMap { pages: None, length: 0 },
                BuddyMap { pages: None, length: 0 },
                BuddyMap { pages: None, length: 0 },
                BuddyMap { pages: None, length: 0 },
                BuddyMap { pages: None, length: 0 },
                BuddyMap { pages: None, length: 0 },
            ],
            bump_start: 0,
            bump_current: 0,
            bump_end: 0,
        }
    }

    fn init(&mut self, bump_start: usize, length: usize) {
        let pages = (length / arch::PAGE_SIZE) + 1;

        self.bump_start = bump_start;
        self.bump_current = bump_start;
        self.bump_end = bump_start + pages * size_of::<Page>();
    }

    fn add_memory_region(&mut self, from: usize, length: usize) {
        if from + length <= self.bump_end {
            return
        }

        let unaligned_from  = if from <= self.bump_end { self.bump_end } else { from };
        let adjusted_from   = utils::round_up_2(unaligned_from, 0x1000);
        let adjusted_length = length - (adjusted_from - from);

        let pages = adjusted_length / arch::PAGE_SIZE;

        let mut page = 0;
        let mut buddy = 5;

        while buddy >= 0 {
            let buddy_length = 2usize.pow(buddy as u32);

            while page + buddy_length < pages {
                for _ in (0..buddy_length) {
                    self.add_page();
                }

                page += buddy_length;
            }

            buddy -= 1;
        }

        early_println!("{} pages from {:x}", pages, adjusted_from);
        early_println!("{:x} to {:x}, current {:x}", self.bump_start, self.bump_end, self.bump_current);
    }

    fn add_page(&mut self) -> *mut Page {
        let page_ptr: *mut Page = self.bump_current as *mut Page;

        self.bump_current += size_of::<Page>();

        if self.bump_current > self.bump_end {
            panic!("More memory pages added to PFA than expected");
        }

        let page = unsafe { page_ptr.as_mut().unwrap() };

        page.use_count = 0;
        page.order = 0;
        page.page_number = 0;
        page.next = None;
        page.next_free = None;

        page_ptr
    }
}
