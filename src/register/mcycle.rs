#[inline]
#[must_use]
pub fn read() -> u64 {
    #[cfg(target_pointer_width = "64")]
    {
        let mcycle: u64;
        unsafe {
            core::arch::asm! {
                "csrr {}, mcycle",
                out(reg) mcycle,
                options(nomem, nostack)
            }
        };
        mcycle
    }

    #[cfg(target_pointer_width = "32")]
    {
        let low: u32;
        let high: u32;

        unsafe {
            core::arch::asm! {
                "1:",
                "csrr {low}, mcycle",
                "csrr {high}, mcycleh",
                "csrr {temp}, mcycleh",
                "bne {high}, {temp}, 1b",
                low = out(reg) low,
                high = out(reg) high,
                temp = out(reg) _,
                options(nomem, nostack)
            }
        };

        ((high as u64) << 32) | (low as u64)
    }
}
