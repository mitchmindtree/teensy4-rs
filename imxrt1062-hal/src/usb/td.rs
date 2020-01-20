//! USB device transfer descriptors (TD)

use super::generic;

#[repr(C, align(32))]
pub struct TD {
    pub next_link_pointer: TD_POINTER,
    pub token: TOKEN,
    pub pointers: [BUFFER_POINTER; 5],
    // There's 4 extra bytes of space here if needed
}

pub type POINTER<T, REG> = generic::Reg<*mut T, REG>;
impl<T, REG> generic::Writable for POINTER<T, REG> {}
impl<T, REG> generic::Readable for POINTER<T, REG> {}
impl<T, REG> generic::ResetValue for POINTER<T, REG> {
    type Type = *mut T;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        core::ptr::null_mut()
    }
}

#[doc(hidden)]
pub struct _TD_POINTER;
pub type TD_POINTER = generic::Reg<u32, _TD_POINTER>;

#[doc(hidden)]
pub struct _BUFFER_POINTER;
pub type BUFFER_POINTER = POINTER<u8, _BUFFER_POINTER>;

#[doc(hidden)]
pub struct _TOKEN;
pub type TOKEN = generic::Reg<u32, _TOKEN>;

pub mod token {
    use super::generic;
    use super::TOKEN;

    pub type W = generic::W<u32, TOKEN>;
    pub type R = generic::R<u32, TOKEN>;

    impl generic::Writable for TOKEN {}
    impl generic::Readable for TOKEN {}

    impl generic::ResetValue for TOKEN {
        type Type = u32;

        #[inline(always)]
        fn reset_value() -> Self::Type {
            0
        }
    }

    const TOTAL_BYTES_SHIFT: u32 = 16;
    const TOTAL_BYTES_MASK: u32 = 0x7FFF;

    pub struct TOTAL_BYTES_W<'w>(&'w mut W);
    impl<'w> TOTAL_BYTES_W<'w> {
        #[inline(always)]
        pub unsafe fn bits(self, value: u16) -> &'w mut W {
            // The maximum recommended transfer is 16K
            let value = value.min(0x4000);
            self.0.bits = (self.0.bits & !(TOTAL_BYTES_MASK << TOTAL_BYTES_SHIFT))
                | (((value as u32) & TOTAL_BYTES_MASK) << TOTAL_BYTES_SHIFT);
            self.0
        }
    }

    const INTERRUPT_ON_COMPLETE_SHIFT: u32 = 15;
    bit_writer!(INTERRUPT_ON_COMPLETE_W, INTERRUPT_ON_COMPLETE_SHIFT);

    pub struct STATUS_W<'w>(&'w mut W);
    bitflags::bitflags! {
        pub struct STATUS_A : u8 {
            const ACTIVE = 1 << 7;
            const HALTED = 1 << 6;
            const DATA_BUFFER_ERROR = 1 << 5;
            const TRANSACTION_ERROR = 1 << 3;
        }
    }
    impl<'w> STATUS_W<'w> {
        #[inline(always)]
        pub fn flags(self, flags: STATUS_A) -> &'w mut W {
            self.0.bits = (self.0.bits & !0xFF) | (flags.bits() as u32);
            self.0
        }
    }

    impl W {
        #[inline(always)]
        pub fn total_bytes(&mut self) -> TOTAL_BYTES_W {
            TOTAL_BYTES_W(self)
        }
        #[inline(always)]
        pub fn interrupt_on_complete(&mut self) -> INTERRUPT_ON_COMPLETE_W {
            INTERRUPT_ON_COMPLETE_W(self)
        }
        #[inline(always)]
        pub fn status(&mut self) -> STATUS_W {
            STATUS_W(self)
        }
    }

    pub type TOTAL_BYTES_R = generic::R<u16, u16>;
    pub type INTERRUPT_ON_COMPLETE_R = generic::R<bool, bool>;
    pub type STATUS_R = generic::R<u8, STATUS_A>;
    impl STATUS_R {
        #[inline(always)]
        pub fn flags(&self) -> STATUS_A {
            STATUS_A::from_bits_truncate(self.bits)
        }
    }

    impl R {
        #[inline(always)]
        pub fn total_bytes(&self) -> TOTAL_BYTES_R {
            TOTAL_BYTES_R::new(((self.bits >> TOTAL_BYTES_SHIFT) & TOTAL_BYTES_MASK) as u16)
        }
        #[inline(always)]
        pub fn interrupt_on_complete(&self) -> INTERRUPT_ON_COMPLETE_R {
            INTERRUPT_ON_COMPLETE_R::new((self.bits & (1 << INTERRUPT_ON_COMPLETE_SHIFT)) > 0)
        }
        #[inline(always)]
        pub fn status(&self) -> STATUS_R {
            STATUS_R::new((self.bits & 0xFF) as u8)
        }
    }
}

pub mod td_pointer {
    use super::generic;
    use super::TD_POINTER;

    pub type W = generic::W<u32, TD_POINTER>;

    impl generic::Writable for TD_POINTER {}

    const NEXT_LINK_POINTER_SHIFT: u32 = 5;
    const NEXT_LINK_POINTER_MASK: u32 = 0x7FFFFFF;

    pub struct NEXT_LINK_POINTER_W<'w>(&'w mut W);
    impl<'w> NEXT_LINK_POINTER_W<'w> {
        /// Set the next transfer descriptor pointer to the provided transfer description pointer
        ///
        /// # Safety
        ///
        /// The memory pointed to by `ptr` must be aligned on a 32-byte boundary.
        #[inline(always)]
        pub unsafe fn ptr(self, ptr: *const super::TD) -> &'w mut W {
            self.0.bits = (self.0.bits & !(NEXT_LINK_POINTER_MASK << NEXT_LINK_POINTER_SHIFT))
                | (((ptr as u32) & NEXT_LINK_POINTER_MASK) << NEXT_LINK_POINTER_SHIFT);
            self.0
        }
    }

    const TERMINATE_SHIFT: u32 = 0;
    bit_writer!(TERMINATE_W, TERMINATE_SHIFT);
    impl<'w> TERMINATE_W<'w> {
        #[inline(always)]
        pub fn invalid(self) -> &'w mut W {
            self.set_bit()
        }
        #[inline(always)]
        pub fn valid(self) -> &'w mut W {
            self.clear_bit()
        }
    }

    impl W {
        #[inline(always)]
        pub fn next_link_pointer(&mut self) -> NEXT_LINK_POINTER_W {
            NEXT_LINK_POINTER_W(self)
        }
        #[inline(always)]
        pub fn terminate(&mut self) -> TERMINATE_W {
            TERMINATE_W(self)
        }
    }
}
