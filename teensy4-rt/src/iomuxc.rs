//! I/O MUX Controller (IOMUXC)
//!
//! The module defines some registers shared
//! throughout other modules in the crate. It's
//! mainly to reduce code duplication...

// Shamelessly lifted from the static_assert crate...
// TODO maybe just depend on it...?
macro_rules! const_assert {
    ($x:expr $(,)?) => {
        #[allow(unknown_lints, clippy::eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = $x;
            ASSERT
        } as usize] = [];
    };
}

/// Base I/O mux control port
pub const IMXRT_IOMUXC: u32 = 0x401F_8000;

pub const IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_03: u32 = IMXRT_IOMUXC + 0x148;
pub const IOMUXC_SW_PAD_CTL_PAD_GPIO_B0_03: u32 = IMXRT_IOMUXC + 0x338;

/// The macro generates a pointer to a GPR register N. Ensures that we're not
/// creating invalid GPR registers.
///
/// This must be called within an `unsafe { ... }` block.
macro_rules! IOMUXC_GPR_GPR {
    ($N:expr) => {{
        // General-Purpose Register (GPR) base address
        const IMXRT_IOMUXC_GPR: *mut u32 = 0x400A_C000 as *mut u32;
        const_assert!($N <= 34);
        IMXRT_IOMUXC_GPR.offset($N)
    }};
}