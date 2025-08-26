#[inline]
#[must_use]
pub fn read() -> usize {
    let sepc: usize;
    unsafe { core::arch::asm!("csrr {}, sepc", out(reg) sepc, options(nomem, nostack)) };
    sepc
}

#[inline]
pub unsafe fn write(sepc: usize) {
    unsafe { core::arch::asm!("csrw sepc, {}", in(reg) sepc & !0b11, options(nomem, nostack)) };
}

#[inline]
pub unsafe fn update(f: impl FnOnce(usize) -> usize) {
    unsafe { write(f(read())) };
}
