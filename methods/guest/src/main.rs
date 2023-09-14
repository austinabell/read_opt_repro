#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

extern crate alloc;

use alloc::vec::Vec;
use core::hint::black_box;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let start = env::get_cycle_count();
    let _data: Vec<u8> = black_box({
        let mut buffer = [0; 400];
        env::read_slice(&mut buffer);
        buffer.to_vec()
    });
    let end = env::get_cycle_count();

    let cycles = (end - start) as u32;
    env::commit(&cycles);
}
