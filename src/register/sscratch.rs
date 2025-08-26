use core::arch::asm;

#[must_use]
#[inline]
pub fn read() -> usize {
    let data: usize;
    unsafe { asm!("csrr {}, sscratch", out(reg) data, options(nomem, nostack)) };
    data
}

#[inline]
pub fn write(data: usize) {
    unsafe { asm!("csrw sscratch, {}", in(reg) data, options(nomem, nostack)) };
}
