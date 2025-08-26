#[inline]
#[must_use]
pub fn read() -> usize {
    let data: usize;
    unsafe { core::arch::asm!("csrr {}, mtvec", out(reg) data, options(nomem, nostack)) };
    data
}

#[inline]
pub fn write() {}
