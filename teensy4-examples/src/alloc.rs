//! Example of heap allocation on the Teensy 4
//!
//! This relies on the 'alloc' feature-flag of the
//! `teensy4-bsp`. The feature-flag adds the `imxrt1062-alloc`
//! crate as a dependency to the BSP crate. The alloc crate
//! provides an implementation of a global allocator that
//! uses the `imxrt1062-rt` symbols.

#![no_main]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
extern crate panic_halt;

use bsp::rt;
use teensy4_bsp as bsp;

#[cfg(feature = "alloc")]
#[rt::entry]
fn main() -> ! {
    // The BSP inititializes the allocator
    let peripherals = bsp::Peripherals::take().unwrap();
    peripherals.log.init(Default::default());
    bsp::delay(5_000);
    log::info!("Allocating vector...");
    bsp::delay(1_000);
    // We should heap memory available by this point
    let mut xs = alloc::vec![0, 1, 2, 3, 4, 5];

    loop {
        log::info!("Collection has {:?} {:p}", xs, xs.as_ptr());
        xs = xs.into_iter().map(|x| x + 1).collect();
        bsp::delay(1_000);
    }
}

//
// This example doesn't do anything unless compiled with the 'alloc' feature
//

#[cfg(not(feature = "alloc"))]
#[rt::entry]
fn main() -> ! {
    let peripherals = bsp::Peripherals::take().unwrap();
    peripherals.log.init(Default::default());
    loop {
        bsp::delay(1_000);
        log::error!(
            "The 'alloc' example doesn't do anything unless compiled with '--features alloc'"
        );
        log::error!("Try compiling this with a nightly compiler and the 'alloc' feature flag");
    }
}
