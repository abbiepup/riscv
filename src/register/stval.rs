use crate::address::Virtual;
use core::arch::asm;

#[must_use]
#[inline]
pub fn read() -> Option<Virtual> {
    let val: usize;
    unsafe { asm!("csrr {}, stval", out(reg) val, options(nomem, nostack)) };
    Virtual::new(val)
}
