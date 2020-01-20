//! Buffers and memory for the USB stack
//!
//! TODO let users define their own storage spaces, rather than
//! allocating static buffers here.

use super::td::TD;
use cortex_m::interrupt::CriticalSection;

/// Max size of a transfer buffer
const TX_SIZE: usize = 2048;
/// Max size of a receive buffer
const RX_SIZE: usize = 512;

/// Number of max-sized TX buffers we're supporting
const TX_NUM: usize = 4;
/// Number of max-sized RX buffers we're supporting
const RX_NUM: usize = 8;

/// Type that describes memory for transfer buffers
#[repr(align(32))]
struct TxBuffer([u8; TX_SIZE * TX_NUM]);

/// Type that describes memory for receiver buffers
#[repr(align(32))]
struct RxBuffer([u8; RX_SIZE * RX_NUM]);

/// Storage space for transfer buffers
#[link_section = ".dmabuffers"]
static mut TX_BUFFER: TxBuffer = TxBuffer([0; TX_SIZE * TX_NUM]);

/// Storage space for receiver buffers
#[link_section = ".dmabuffers"]
static mut RX_BUFFER: RxBuffer = RxBuffer([0; RX_SIZE * RX_NUM]);

/// Access TX transfer descriptors
pub fn tx_transfer_descriptors<R, F>(_cs: &CriticalSection, f: F) -> R
where
    F: FnOnce(&[TD]) -> R,
{
    static TRANSFER_DESCRIPTORS: [u8; TX_NUM * core::mem::size_of::<TD>()] =
        [0; TX_NUM * core::mem::size_of::<TD>()];
    let tds =
        unsafe { core::slice::from_raw_parts(TRANSFER_DESCRIPTORS.as_ptr() as *const TD, TX_NUM) };
    f(tds)
}

/// Access RX transfer descriptors
pub fn rx_transfer_descriptors<R, F>(_cs: &CriticalSection, f: F) -> R
where
    F: FnOnce(&[TD]) -> R,
{
    static TRANSFER_DESCRIPTORS: [u8; RX_NUM * core::mem::size_of::<TD>()] =
        [0; RX_NUM * core::mem::size_of::<TD>()];
    let tds =
        unsafe { core::slice::from_raw_parts(TRANSFER_DESCRIPTORS.as_ptr() as *const TD, RX_NUM) };
    f(tds)
}

/// A buffer usable for endpoint reads and writes
pub struct Buffer(&'static mut [vcell::VolatileCell<u8>]);

impl Buffer {
    pub fn read(&self, buffer: &mut [u8]) -> usize {
        for (dst, src) in buffer.iter_mut().zip(self.0.iter()) {
            *dst = src.get();
        }
        buffer.len().min(self.0.len())
    }

    pub fn write(&mut self, buffer: &[u8]) -> usize {
        for (dst, src) in self.0.iter_mut().zip(buffer.iter()) {
            dst.set(*src);
        }
        buffer.len().min(self.0.len())
    }

    pub fn capacity(&self) -> usize {
        self.0.len()
    }
}

/// A buffer allocator
///
/// Returns `Buffer`s that access back into static storage.
pub struct Allocator {
    /// Current offset into the source
    offset: usize,
    /// Source buffer
    source: &'static mut [u8],
    /// Alignment of a buffer allocated from this allocator
    align: usize,
}

impl Allocator {
    unsafe fn new(source: &'static mut [u8], align: usize) -> Self {
        Allocator {
            offset: source.len(),
            source,
            align,
        }
    }

    /// Create an allocator for transfer buffers
    ///
    /// Safety: should only be called once
    pub unsafe fn tx() -> Allocator {
        Allocator::new(&mut TX_BUFFER.0, TX_SIZE)
    }

    /// Create an allocator for receive buffers
    ///
    /// Safety: should only be called once
    pub unsafe fn rx() -> Allocator {
        Allocator::new(&mut RX_BUFFER.0, RX_SIZE)
    }

    /// Allocate a buffer of size `size`. Returns a buffer if there's
    /// enough space, or `None` if the backing memory cannot support a
    /// buffer of that size.
    pub fn alloc(&mut self, size: usize) -> Option<Buffer> {
        self.offset = self.offset.checked_sub(size)? & !(self.align - 1);
        let buffer = Buffer(unsafe {
            core::slice::from_raw_parts_mut(
                self.source.as_mut_ptr().add(self.offset) as *mut _,
                size,
            )
        });
        Some(buffer)
    }
}
