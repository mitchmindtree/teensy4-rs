//! USB endpoints

use super::buffer;
use super::qh;

use cortex_m::interrupt::{CriticalSection, Mutex};
use imxrt1062_pac as pac;
use usb_device::{endpoint::EndpointAddress, endpoint::EndpointType, UsbDirection};

pub const NUMBER_OF_ENDPOINTS: usize = 8;
const NUMBER_OF_QUEUE_HEADS: usize = 2 * NUMBER_OF_ENDPOINTS;

/// The list of endpoint queue heads. It must be aligned to a 4096
/// page boundary.
#[repr(align(4096))]
struct QueueHeadList([u8; core::mem::size_of::<qh::QH>() * NUMBER_OF_QUEUE_HEADS]);

/// Actual allocation of queue heads
static QUEUE_HEAD_LIST: QueueHeadList =
    QueueHeadList([0; core::mem::size_of::<qh::QH>() * NUMBER_OF_QUEUE_HEADS]);

/// Provides read/write access to the endpoint queue heads
fn queue_heads<R, F>(_cs: &CriticalSection, f: F) -> R
where
    F: FnOnce(&[qh::QH]) -> R,
{
    // Safety: no mutable aliasing since we're inside a critical
    // section. Caller cannot capture the queue head reference.
    // The size assumptions line up and are guaranteed by the math
    // in this module.
    let eps = unsafe {
        core::slice::from_raw_parts(
            QUEUE_HEAD_LIST.0.as_ptr() as *const qh::QH,
            NUMBER_OF_QUEUE_HEADS,
        )
    };
    f(eps)
}

/// Computes the index of a queue head from an endpoint index and
/// USB direction.
fn ep_to_qh(addr: &EndpointAddress) -> usize {
    let offset = match addr.direction() {
        UsbDirection::Out => 0,
        UsbDirection::In => 1,
    };
    (addr.index() * 2) + offset
}

/// Error indicating that this is a control-only endpoint
#[derive(Debug)]
pub enum AllocationError {
    ControlOnly,
    OutOfBounds,
    OutOfMemory,
}

impl From<AllocationError> for usb_device::UsbError {
    fn from(ae: AllocationError) -> usb_device::UsbError {
        match ae {
            AllocationError::ControlOnly => usb_device::UsbError::InvalidEndpoint,
            AllocationError::OutOfBounds | AllocationError::OutOfMemory => {
                usb_device::UsbError::EndpointOverflow
            }
        }
    }
}

type Mask = u16;

pub struct Endpoints {
    /// Bitmask indicating if an endpoint's TX or RX interface is enabled. This is
    /// maintained separately from the register, since we cannot
    /// set the endpoint to 'enabled' until the host issues set configuration.
    mask: Mask,
    /// An endpoint's TX or RX buffer. Indexed by a queue head index
    buffers: [Option<Mutex<buffer::Buffer>>; NUMBER_OF_QUEUE_HEADS],
    /// TX buffer allocator
    tx_alloc: buffer::Allocator,
    /// RX buffer allocator
    rx_alloc: buffer::Allocator,
}

impl Endpoints {
    pub fn new() -> Self {
        Endpoints {
            mask: 0,
            buffers: {
                use core::mem::MaybeUninit;
                let mut buffers: [MaybeUninit<Option<Mutex<buffer::Buffer>>>;
                    NUMBER_OF_QUEUE_HEADS] = unsafe { MaybeUninit::uninit().assume_init() };
                for buffer in buffers.iter_mut() {
                    *buffer = MaybeUninit::new(None);
                }
                unsafe { core::mem::transmute(buffers) }
            },
            tx_alloc: unsafe { buffer::Allocator::tx() },
            rx_alloc: unsafe { buffer::Allocator::rx() },
        }
    }

    fn mask_offset(&self, addr: &EndpointAddress) -> usize {
        // If this check fails, the type that's used to represent the allocation
        // mask cannot satisfy the number of endpoints that we expose.
        const _STATIC_ASSERT_MASK_SUPPORTS_ENDPOINTS: [u8; 1] =
            [0; ((core::mem::size_of::<Mask>() * 8 / 2) == NUMBER_OF_ENDPOINTS) as usize];

        let offset = match addr.direction() {
            UsbDirection::In => 0usize,
            UsbDirection::Out => core::mem::size_of_val(&self.mask) * 8 / 2,
        };
        addr.index() + offset
    }

    pub fn is_allocated(&self, _cs: &CriticalSection, addr: &EndpointAddress) -> bool {
        let offset = self.mask_offset(addr);
        (self.mask & (1 << offset) > 0)
    }

    fn setup_endpoint(
        &self,
        reg: &pac::usb1::ENDPTCTRL,
        ep: &EndpointDefinition,
    ) -> Result<(), AllocationError> {
        reg.write(|w| unsafe {
            // Safety: enum raw representation matches what the hardware requires
            match ep.addr.direction() {
                UsbDirection::In => w.txt().bits(ep.ep_type as u8),
                UsbDirection::Out => w.rxt().bits(ep.ep_type as u8),
            }
        });
        Ok(())
    }

    pub fn allocate(
        &mut self,
        cs: &CriticalSection,
        reg: &pac::USB1,
        ep: &EndpointDefinition,
    ) -> Result<(), AllocationError> {
        match ep.addr.index() {
            0 if ep.ep_type == EndpointType::Control => {
                // Nothing special to do.
                // Hardware already configures EP0.
            }
            0 if ep.ep_type != EndpointType::Control => return Err(AllocationError::ControlOnly),
            1 => self.setup_endpoint(&reg.endptctrl1, ep)?,
            2 => self.setup_endpoint(&reg.endptctrl2, ep)?,
            3 => self.setup_endpoint(&reg.endptctrl3, ep)?,
            4 => self.setup_endpoint(&reg.endptctrl4, ep)?,
            5 => self.setup_endpoint(&reg.endptctrl5, ep)?,
            6 => self.setup_endpoint(&reg.endptctrl6, ep)?,
            7 => self.setup_endpoint(&reg.endptctrl7, ep)?,
            _ => return Err(AllocationError::OutOfBounds),
        };

        let qh_idx = ep_to_qh(&ep.addr);
        queue_heads(cs, |qhs| {
            qhs[qh_idx].config.write(|w| {
                w.max_packet_length()
                    .bits(ep.max_packet_size)
                    .interrupt_on_setup()
                    .bit(
                        UsbDirection::Out == ep.addr.direction()
                            && EndpointType::Control == ep.ep_type,
                    )
            });
        });

        let alloc = match ep.addr.direction() {
            UsbDirection::In => &mut self.tx_alloc,
            UsbDirection::Out => &mut self.rx_alloc,
        };

        self.buffers[qh_idx] = Some(
            alloc
                .alloc(ep.max_packet_size as usize)
                .map(Mutex::new)
                .ok_or(AllocationError::OutOfMemory)?,
        );

        let offset = self.mask_offset(&ep.addr);
        self.mask |= 1 << offset;
        Ok(())
    }
}

pub struct EndpointDefinition {
    pub addr: EndpointAddress,
    pub max_packet_size: u16,
    pub ep_type: EndpointType,
}
