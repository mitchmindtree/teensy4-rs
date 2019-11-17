//! The example simply enables the LED

#![no_std]
#![no_main]

use cortex_m::asm::wfi;
use teensy4_rt::{enable_led, entry};

#[entry]
fn main() -> ! {
    loop {
        enable_led();
        wfi();
    }
}