use core::arch::asm;
use core::ptr::NonNull;

/// Atomic fetch-and-add doubleword.
#[inline]
#[doc(alias = "amoadd.d")]
#[cfg(target_arch = "riscv64")]
#[target_feature(enable = "a")]
pub unsafe fn amoadd_d(value: u64, address: NonNull<u64>) -> u64 {
    let old;
    unsafe { asm!("amoadd.d {}, {}, ({})", out(reg) old, in(reg) value, in(reg) address.as_ptr(), options(nostack)) };
    old
}

/// Atomic fetch-and-add word.
#[inline]
#[doc(alias = "amoadd.w")]
#[target_feature(enable = "a")]
pub unsafe fn amoadd_w(value: u32, address: NonNull<u32>) -> u32 {
    let old;
    unsafe { asm!("amoadd.w {}, {}, ({})", out(reg) old, in(reg) value, in(reg) address.as_ptr(), options(nostack)) };
    old
}

/// Atomic fetch-and-and word.
#[inline]
#[doc(alias = "amoand.w")]
#[target_feature(enable = "a")]
pub unsafe fn amoand_w(value: u32, address: NonNull<u32>) -> u32 {
    let old;
    unsafe { asm!("amoand.w {}, {}, ({})", out(reg) old, in(reg) value, in(reg) address.as_ptr(), options(nostack)) }
    old
}

/// Atomic fetch-and-or word.
#[inline]
#[doc(alias = "amoor.w")]
#[target_feature(enable = "a")]
pub unsafe fn amoor_w(value: u32, address: NonNull<u32>) -> u32 {
    let old;
    unsafe { asm!("amoor.w {}, {}, ({})", lateout(reg) old, in(reg) value, in(reg) address.as_ptr(), options(nostack)) }
    old
}

/// Atomic fetch-and-swap word.
#[inline]
#[doc(alias = "amoswap.w")]
#[target_feature(enable = "a")]
pub unsafe fn amoswap_w(value: u32, address: NonNull<u32>) -> u32 {
    let old;
    unsafe { asm!("amoswap.w {}, {}, ({})", lateout(reg) old, in(reg) value, in(reg) address.as_ptr(), options(nostack)) };
    old
}

/// Atomic fetch-and-xor word.
#[inline]
#[doc(alias = "amoxor.w")]
#[target_feature(enable = "a")]
pub unsafe fn amoxor_w(value: u32, address: NonNull<u32>) -> u32 {
    let old;
    unsafe { asm!("amoxor.w {}, {}, ({})", lateout(reg) old, in(reg) value, in(reg) address.as_ptr(), options(nostack)) };
    old
}

/// Load-reserved word.
#[inline]
#[doc(alias = "lr.w")]
#[target_feature(enable = "a")]
pub unsafe fn lr_w(address: NonNull<u32>) -> u32 {
    let value;
    unsafe { asm!("lr.w {}, ({})", lateout(reg) value, in(reg) address.as_ptr(), options(nostack)) }
    value
}

/// Store-conditional word.
#[inline]
#[doc(alias = "sc.w")]
#[target_feature(enable = "a")]
pub unsafe fn sc_w(value: u32, address: NonNull<u32>) -> u32 {
    let status: u32;
    unsafe { asm!("sc.w {}, {}, ({})", lateout(reg) status, in(reg) value, in(reg) address.as_ptr(), options(nostack)) }
    status
}
