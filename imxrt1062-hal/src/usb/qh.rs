//! USB device endpoint queue head

use super::generic;

#[repr(C, align(64))] // Must be aligned on a 64-byte boundary
pub struct QH {
    pub config: CONFIG,
    // TODO the rest
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

    bit_writer!(ZERO_LENGTH_TERMINATION_W, 29);
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

    pub struct MAX_PACKET_LENGTH<'w>(&'w mut W);
    impl<'w> MAX_PACKET_LENGTH<'w> {
        /// Set the max packet length. This is clamped at 1024, the maximum packet size.
        #[inline(always)]
        pub fn bits(self, value: u16) -> &'w mut W {
            let value = core::cmp::min(value, 0x400);
            self.0.bits = (self.0.bits & !(0x7FF << 16)) | (((value as u32) & 0x7FF) << 16);
            self.0
        }
    }

    bit_writer!(INTERRUPT_ON_SETUP_W, 15);

    impl W {
        #[inline(always)]
        pub fn max_packet_length(&mut self) -> MAX_PACKET_LENGTH {
            MAX_PACKET_LENGTH(self)
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
}