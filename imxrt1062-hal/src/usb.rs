//! USB device support

#![allow(non_camel_case_types)]

#[macro_use]
mod generic;

mod qh;

use crate::pac;

/// Initialize the USB PLL
///
/// TODO hide this inside of a USB struct constructor. This is exposed
/// to maintain compatibility with the C USB implementation. Once the
/// entire thing is in Rust, this shouldn't be exposed
pub fn pll_init(ccm_analog: &pac::CCM_ANALOG) {
    loop {
        let pll = ccm_analog.pll_usb1.read();
        if pll.div_select().bit_is_set() {
            // We're in 528MHz mode, which is atypical
            ccm_analog
                .pll_usb1_clr
                .write_with_zero(|w| w.bypass_clk_src().ref_clk_24m());
            ccm_analog
                .pll_usb1_set
                .write_with_zero(|w| w.bypass().set_bit());
            ccm_analog.pll_usb1_clr.write_with_zero(|w| {
                w.power()
                    .set_bit()
                    .div_select()
                    .set_bit()
                    .enable()
                    .set_bit()
                    .en_usb_clks()
                    .set_bit()
            });
            continue;
        }

        if pll.enable().bit_is_clear() {
            ccm_analog
                .pll_usb1_set
                .write_with_zero(|w| w.enable().set_bit());
            continue;
        }

        if pll.power().bit_is_clear() {
            ccm_analog
                .pll_usb1_set
                .write_with_zero(|w| w.power().set_bit());
            continue;
        }

        if pll.lock().bit_is_clear() {
            continue; // Just need to wait for the lock
        }

        if pll.bypass().bit_is_set() {
            ccm_analog
                .pll_usb1_clr
                .write_with_zero(|w| w.bypass().set_bit());
            continue;
        }

        if pll.en_usb_clks().bit_is_clear() {
            ccm_analog
                .pll_usb1_set
                .write_with_zero(|w| w.en_usb_clks().set_bit());
            continue;
        }

        break;
    }
}
