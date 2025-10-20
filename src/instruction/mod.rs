use core::arch::asm;
use core::ptr::NonNull;

mod vsetvli;
pub use vsetvli::*;

/// Generates the `nop` instruction.
#[inline]
pub fn nop() {
    unsafe { asm!("nop", options(nomem, nostack, preserves_flags)) };
}

/// Wait for interrupt.
///
/// Can causes the processor to enter a low-power state until the next interrupt occurs.
#[inline]
pub unsafe fn wfi() {
    unsafe { asm!("wfi", options(nomem, nostack, preserves_flags)) };
}

/// Memory ordering fence.
#[inline]
pub unsafe fn fence() {
    unsafe { asm!("fence", options(nostack, preserves_flags)) };
}

#[inline]
#[target_feature(enable = "zifencei")]
pub unsafe fn fence_i() {
    unsafe { asm!("fence.i", options(nostack, preserves_flags)) };
}

/// Supervisor memory-management fence.
#[inline]
pub fn sfence_vma(address: usize, asid: usize) {
    unsafe { asm!("sfence.vma {} {}", in(reg) address, in(reg) asid, options(nostack, preserves_flags)) };
}

/// Environment call.
#[inline]
pub fn ecall() {
    unsafe { asm!("ecall", options(nomem, nostack)) };
}

/// Breakpoint exception.
#[inline]
pub fn ebreak() {
    unsafe { asm!("ebreak", options(nomem, nostack)) };
}

/// Atomic fetch-and-add word.
#[inline]
#[target_feature(enable = "a")]
pub unsafe fn amoadd_w(value: u32, address: *mut u32) -> u32 {
    let old;
    unsafe { asm!("amoadd.w {}, {}, ({})", out(reg) old, in(reg) value, in(reg) address, options(nostack, preserves_flags)) };
    old
}

/// Atomic fetch-and-add doubleword.
#[inline]
#[cfg(target_arch = "riscv64")]
#[target_feature(enable = "a")]
pub unsafe fn amoadd_d(value: u64, address: *mut u64) -> u64 {
    let old;
    unsafe { asm!("amoadd.d {}, {}, ({})", out(reg) old, in(reg) value, in(reg) address, options(nostack, preserves_flags)) };
    old
}

/// Compressed add.
#[inline]
#[target_feature(enable = "c")]
pub fn c_add(lhs: usize, rhs: usize) -> usize {
    let result;
    unsafe { asm!("c.add {}, {}", inlateout(reg) lhs => result, in(reg) rhs, options(pure, nomem, nostack)) };
    result
}

/// AES middle round encryption instruction for RV64.
#[inline]
#[cfg(target_arch = "riscv64")]
#[target_feature(enable = "zkne")]
pub fn aes64esm(rs1: u64, rs2: u64) -> u64 {
    let result;
    unsafe { asm!("aes64esm {}, {}, {}", lateout(reg) result, in(reg) rs1, in(reg) rs2, options(pure, nomem, nostack)) };
    result
}

#[inline]
#[cfg(target_arch = "riscv64")]
#[target_feature(enable = "zknd")]
pub fn aes64dsm(rs1: u64, rs2: u64) -> u64 {
    let result;
    unsafe { asm!("aes64dsm {}, {}, {}", lateout(reg) result, in(reg) rs1, in(reg) rs2, options(pure, nomem, nostack)) };
    result
}

/// Bitware OR-combine, byte granule.
#[inline]
#[target_feature(enable = "zbb")]
pub fn orc_b(rs: usize) -> usize {
    let result;
    unsafe { asm!("orc.b {}, {}", lateout(reg) result, in(reg) rs, options(pure, nomem, nostack)) };
    result
}

#[inline]
#[target_feature(enable = "zicbom")]
pub fn cbo_zero(address: *mut usize) {
    unsafe { asm!("cbo.zero {}", in(reg) address, options(nostack)) };
}

/// Count leading zero bits.
///
/// This instruction counts the number of 0’s before the first 1, starting at the most-significant bit (i.e., XLEN-1)
/// and progressing to bit 0. Accordingly, if the input is 0, the output is XLEN, and if the most-significant bit of
/// the input is a 1, the output is 0.
#[inline]
#[target_feature(enable = "zbb")]
pub fn clz(value: usize) -> usize {
    let result;
    unsafe { asm!("clz {}, {}", out(reg) result, in(reg) value, options(pure, nomem, nostack)) };
    result
}

/// Count leading zero bits in word.
///
/// This instruction counts the number of 0’s before the first 1 starting at bit 31 and progressing to bit 0.
/// Accordingly, if the least-significant word is 0, the output is 32, and if the most-significant bit of the word
/// (i.e., bit 31) is a 1, the output is 0.
#[inline]
#[target_feature(enable = "zbb")]
pub fn clzw(value: u32) -> u32 {
    let result;
    unsafe { asm!("clzw {}, {}", out(reg) result, in(reg) value, options(pure, nomem, nostack)) };
    result
}

/// Count set bits.
///
/// This instructions counts the number of 1’s (i.e., set bits) in the source register.
#[inline]
#[target_feature(enable = "zbb")]
pub fn cpop(value: usize) -> usize {
    let result;
    unsafe { asm!("cpop {}, {}", out(reg) result, in(reg) value, options(pure, nomem, nostack)) };
    result
}

/// Count set bits in word.
///
/// This instructions counts the number of 1’s (i.e., set bits) in the least-significant word of the source register.
#[inline]
#[target_feature(enable = "zbb")]
pub fn cpopw(value: u32) -> u32 {
    let result;
    unsafe { asm!("cpopw {}, {}", out(reg) result, in(reg) value, options(pure, nomem, nostack)) };
    result
}

/// Atomic fetch-and-swap word.
#[inline]
#[target_feature(enable = "a")]
pub fn amoswap_w(value: u32, address: NonNull<u32>) -> u32 {
    let old;
    unsafe { asm!("amoswap.w {}, {}, ({})", out(reg) old, in(reg) value, in(reg) address.as_ptr(), options(nostack)) };
    old
}
