#[inline]
#[must_use]
pub fn read() -> usize {
    let bits: usize;
    unsafe { core::arch::asm!("csrr {}, sstatus", out(reg) bits, options(nomem, nostack)) };
    bits
}

#[inline]
pub unsafe fn write(bits: usize) {
    unsafe { core::arch::asm!("csrw sstatus, {}", in(reg) bits, options(nomem, nostack)) };
}
