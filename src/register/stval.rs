use crate::address::Virtual;
use core::arch::asm;

#[inline]
#[must_use]
pub fn read() -> Option<Virtual> {
    let val: usize;
    unsafe { asm!("csrr {}, stval", out(reg) val, options(nomem, nostack)) };
    Virtual::new(val)
}
