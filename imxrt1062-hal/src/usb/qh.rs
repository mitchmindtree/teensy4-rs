//! USB device endpoint queue head

use super::generic;
use super::td;

#[repr(C, align(64))] // Must be aligned on a 64-byte boundary
pub struct QH {
    pub config: CONFIG,
    pub current: td::TD_POINTER,
    pub next: td::TD_POINTER,
    pub token: td::TOKEN,
    _pointers: [u32; 5],
    _reserved: u32,
    pub setup: [u8; 8],
}

#[doc(hidden)]
pub struct _CONFIG;
pub type CONFIG = generic::Reg<u32, _CONFIG>;

/// Implementations for the config field of the queue head
pub mod config {
    use super::generic;
    use super::CONFIG;

    impl generic::Writable for CONFIG {}
    impl generic::Readable for CONFIG {}

    pub type W = generic::W<u32, CONFIG>;
    pub type R = generic::R<u32, CONFIG>;

    impl generic::ResetValue for super::CONFIG {
        type Type = u32;
        #[inline(always)]
        fn reset_value() -> Self::Type {
            0
        }
    }

    const ZERO_LENGTH_TERMINATION_SHIFT: u32 = 29;
    bit_writer!(ZERO_LENGTH_TERMINATION_W, ZERO_LENGTH_TERMINATION_SHIFT);
    impl<'w> ZERO_LENGTH_TERMINATION_W<'w> {
        #[inline(always)]
        pub fn enable(self) -> &'w mut W {
            self.clear_bit()
        }

        #[inline(always)]
        pub fn disable(self) -> &'w mut W {
            self.set_bit()
        }
    }

    const MAX_PACKET_LENGTH_MASK: u32 = 0x7FF;
    const MAX_PACKET_LENGTH_SHIFT: u32 = 16;

    pub struct MAX_PACKET_LENGTH_W<'w>(&'w mut W);
    impl<'w> MAX_PACKET_LENGTH_W<'w> {
        /// Set the max packet length. This is clamped at 1024, the maximum packet size.
        #[inline(always)]
        pub fn bits(self, value: u16) -> &'w mut W {
            let value = value.min(0x400);
            self.0.bits = (self.0.bits & !(MAX_PACKET_LENGTH_MASK << MAX_PACKET_LENGTH_SHIFT))
                | (((value as u32) & MAX_PACKET_LENGTH_MASK) << MAX_PACKET_LENGTH_SHIFT);
            self.0
        }
    }

    const INTERRUPT_ON_SETUP_SHIFT: u32 = 15;
    bit_writer!(INTERRUPT_ON_SETUP_W, INTERRUPT_ON_SETUP_SHIFT);

    impl W {
        #[inline(always)]
        pub fn max_packet_length(&mut self) -> MAX_PACKET_LENGTH_W {
            MAX_PACKET_LENGTH_W(self)
        }
        #[inline(always)]
        pub fn interrupt_on_setup(&mut self) -> INTERRUPT_ON_SETUP_W {
            INTERRUPT_ON_SETUP_W(self)
        }
        #[inline(always)]
        pub fn zero_length_termination(&mut self) -> ZERO_LENGTH_TERMINATION_W {
            ZERO_LENGTH_TERMINATION_W(self)
        }
    }

    #[doc(hidden)]
    pub struct _ZERO_LENGTH_TERMINATION;
    pub type ZERO_LENGTH_TERMINATION_R = generic::R<bool, _ZERO_LENGTH_TERMINATION>;
    impl ZERO_LENGTH_TERMINATION_R {
        pub fn is_enabled(&self) -> bool {
            self.bit_is_clear()
        }
        pub fn is_disabled(&self) -> bool {
            self.bit_is_set()
        }
    }

    pub type MAX_PACKET_LENGTH_R = generic::R<u16, u16>;
    pub type INTERRUPT_ON_SETUP_R = generic::R<bool, bool>;
    impl R {
        #[inline(always)]
        pub fn max_packet_length(&self) -> MAX_PACKET_LENGTH_R {
            MAX_PACKET_LENGTH_R::new(
                ((self.bits >> MAX_PACKET_LENGTH_SHIFT) & MAX_PACKET_LENGTH_MASK) as u16,
            )
        }
        #[inline(always)]
        pub fn interrupt_on_setup(&self) -> INTERRUPT_ON_SETUP_R {
            INTERRUPT_ON_SETUP_R::new((self.bits & (1 << INTERRUPT_ON_SETUP_SHIFT)) > 0)
        }
        #[inline(always)]
        pub fn zero_length_termination(&self) -> ZERO_LENGTH_TERMINATION_R {
            ZERO_LENGTH_TERMINATION_R::new((self.bits & (1 << ZERO_LENGTH_TERMINATION_SHIFT)) > 0)
        }
    }
}
