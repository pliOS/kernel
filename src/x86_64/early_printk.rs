// Copyright 2016 The pliOS Developers. See the LICENSE
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>. This file may not
// be copied, modified, or distributed except according
// to these terms.

use core::fmt::{self, Write};
use spin::Mutex;
use x86::shared::io;

pub static COM1: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x3F8));

pub fn init() {
    COM1.lock().init();
}

pub fn print(args: fmt::Arguments) {
    COM1.lock().write_fmt(args).unwrap();
}

macro_rules! early_print {
    ($($arg:tt)*) => ({
        $crate::arch::early_printk::print(format_args!($($arg)*));
    });
}

macro_rules! early_println {
    ($fmt:expr) => (early_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (early_print!(concat!($fmt, "\n"), $($arg)*));
}

pub struct SerialPort {
    base: u16,
}

impl SerialPort {
    const fn new(base: u16) -> SerialPort {
        SerialPort {
            base: base,
        }
    }

    fn init(&self) {
        self.outb(1, 0x00);
        self.outb(3, 0x80);
        self.outb(0, 0x03);
        self.outb(1, 0x00);
        self.outb(3, 0x03);
        self.outb(2, 0xC7);
        self.outb(4, 0x0B);
    }

    fn putc(&self, ch: u8) {
        while (self.inb(5) & 0x20) == 0 {}

        self.outb(0, ch)
    }

    fn outb(&self, offset: u16, data: u8) {
        unsafe {
            io::outb(self.base + offset, data)
        }
    }

    fn inb(&self, offset: u16) -> u8 {
        unsafe {
            io::inb(self.base + offset)
        }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }

        Ok(())
    }
}
