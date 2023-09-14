#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use core::hint::black_box;
use core::ptr::null_mut;
use risc0_zkvm::guest::env;
use risc0_zkvm_platform::fileno;
use risc0_zkvm_platform::syscall::{nr, syscall_1, Return};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let start = env::get_cycle_count();
    let _data: Vec<u8> = black_box({
        // SAFETY: This is safe because it is currently a valid host function for the risc0 VM.
        //         Returns the number of bytes available to read with, and an invalid value returned
        //         will just error in the `read_slice` call below, with no UB.
        let Return(navail, _) =
            unsafe { syscall_1(nr::SYS_READ_AVAIL, null_mut(), 0, fileno::STDIN) };

        // Initialize buffer with capacity of stdin input length.
        // NOTE: Ideally this doesn't zero out all bytes before reading, but
        // to just use read_slice and not use more low-level and unsafe code,
        // this is being done.
        let mut buffer = vec![0u8; navail as usize];

        env::read_slice(&mut buffer);
        buffer
    });
    let end = env::get_cycle_count();

    let cycles = (end - start) as u32;
    env::commit(&cycles);
}
