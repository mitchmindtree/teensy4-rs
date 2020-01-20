//! USB device support

#![allow(non_camel_case_types)]

#[macro_use]
mod generic;

mod buffer;
mod endpoint;
mod qh;
mod td;

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

use cortex_m::interrupt::{self, Mutex};

pub struct UsbBus {
    peripheral: Mutex<pac::USB1>,
    endpoints: endpoint::Endpoints,
    tx_alloc: buffer::Allocator,
    rx_alloc: buffer::Allocator,
}

impl UsbBus {
    pub fn new(peripheral: pac::USB1) -> usb_device::bus::UsbBusAllocator<Self> {
        let bus = UsbBus {
            peripheral: Mutex::new(peripheral),
            tx_alloc: unsafe { buffer::Allocator::tx() },
            rx_alloc: unsafe { buffer::Allocator::rx() },
            endpoints: endpoint::Endpoints::new(),
        };
        usb_device::bus::UsbBusAllocator::new(bus)
    }
}

use usb_device::{
    bus::{self, PollResult},
    endpoint::{EndpointAddress, EndpointType},
    Result, UsbDirection,
};

impl bus::UsbBus for UsbBus {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> Result<EndpointAddress> {
        interrupt::free(|cs| {
            if let Some(addr) = ep_addr {
                if !self.endpoints.is_allocated(cs, &addr) {
                    let reg = self.peripheral.borrow(cs);
                    self.endpoints.allocate(
                        cs,
                        reg,
                        &endpoint::EndpointDefinition {
                            addr: addr.clone(),
                            max_packet_size,
                            ep_type,
                        },
                    )?;
                    Ok(addr)
                } else {
                    Err(usb_device::UsbError::InvalidEndpoint)
                }
            } else {
                for addr in (1..endpoint::NUMBER_OF_ENDPOINTS)
                    .map(|idx| EndpointAddress::from_parts(idx, ep_dir))
                    .filter(|addr| !self.endpoints.is_allocated(cs, addr))
                {
                    let reg = self.peripheral.borrow(cs);
                    self.endpoints.allocate(
                        cs,
                        reg,
                        &endpoint::EndpointDefinition {
                            addr: addr.clone(),
                            max_packet_size,
                            ep_type,
                        },
                    )?;
                    return Ok(addr);
                }
                return Err(usb_device::UsbError::EndpointOverflow);
            }
        })
    }
    fn enable(&mut self) {
        todo!()
    }
    fn reset(&self) {
        todo!()
    }
    fn set_device_address(&self, addr: u8) {
        todo!()
    }
    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize> {
        todo!()
    }
    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize> {
        todo!()
    }
    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        todo!()
    }
    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        todo!()
    }
    fn suspend(&self) {
        todo!()
    }
    fn resume(&self) {
        todo!()
    }
    fn poll(&self) -> PollResult {
        todo!()
    }
}
