#![feature(asm)]
use std::asm;

fn main() {
    unsafe {
        asm!("int 42");
    }
}
