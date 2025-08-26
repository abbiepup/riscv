use core::arch::asm;
use core::ptr::NonNull;
mod atomic;
mod vsetvli;
pub use atomic::*;
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
#[doc(alias = "sfence.vma")]
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

/// AES middle round decryption instruction for RV64.
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

/// Cache block zero.
#[inline]
#[target_feature(enable = "zicbom")]
pub fn cbo_zero(address: NonNull<usize>) {
    unsafe { asm!("cbo.zero {}", in(reg) address.as_ptr(), options(nostack)) };
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
    unsafe { asm!("clz {}, {}", lateout(reg) result, in(reg) value, options(pure, nomem, nostack)) };
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
    unsafe { asm!("clzw {}, {}", lateout(reg) result, in(reg) value, options(pure, nomem, nostack)) };
    result
}

/// Count set bits.
///
/// This instructions counts the number of 1’s (i.e., set bits) in the source register.
#[inline]
#[target_feature(enable = "zbb")]
pub fn cpop(value: usize) -> usize {
    let result;
    unsafe { asm!("cpop {}, {}", lateout(reg) result, in(reg) value, options(pure, nomem, nostack)) };
    result
}

/// Count set bits in word.
///
/// This instructions counts the number of 1’s (i.e., set bits) in the least-significant word of the source register.
#[inline]
#[target_feature(enable = "zbb")]
pub fn cpopw(value: u32) -> u32 {
    let result;
    unsafe { asm!("cpopw {}, {}", lateout(reg) result, in(reg) value, options(pure, nomem, nostack)) };
    result
}

#[inline]
pub fn not(value: usize) -> usize {
    let result;
    unsafe { asm!("not {}, {}", lateout(reg) result, in(reg) value, options(nostack)) }
    result
}

/// Exclusive or.
#[inline]
pub fn xor(lhs: usize, rhs: usize) -> usize {
    let result;
    unsafe { asm!("xor {}, {}, {}", lateout(reg) result, in(reg) lhs, in(reg) rhs, options(pure, nomem, nostack)) }
    result
}

#[inline]
pub fn slt(lhs: usize, rhs: usize) -> usize {
    let result;
    unsafe { asm!("slt {}, {}, {}", lateout(reg) result, in(reg) lhs, in(reg) rhs, options(pure, nomem, nostack)) }
    result
}

/// Integer add.
#[inline]
pub fn add(lhs: usize, rhs: usize) -> usize {
    let result;
    unsafe { asm!("add {}, {}, {}", lateout(reg) result, in(reg) lhs, in(reg) rhs, options(pure, nomem, nostack)) }
    result
}

/// Subtract.
#[inline]
pub fn sub(lhs: usize, rhs: usize) -> usize {
    let result;
    unsafe { asm!("sub {}, {}, {}", lateout(reg) result, in(reg) lhs, in(reg) rhs, options(pure, nomem, nostack)) }
    result
}

/// Signed multiply.
#[inline]
#[target_feature(enable = "m")]
pub fn mul(lhs: usize, rhs: usize) -> usize {
    let result;
    unsafe { asm!("sub {}, {}, {}", lateout(reg) result, in(reg) lhs, in(reg) rhs, options(pure, nomem, nostack)) }
    result
}

/// Signed division.
#[inline]
#[target_feature(enable = "m")]
pub fn div(lhs: usize, rhs: usize) -> usize {
    let result;
    unsafe { asm!("div {}, {}, {}", lateout(reg) result, in(reg) lhs, in(reg) rhs, options(pure, nomem, nostack)) }
    result
}

#[inline]
#[target_feature(enable = "m")]
pub fn rem(lhs: usize, rhs: usize) -> usize {
    let result;
    unsafe { asm!("rem {}, {}, {}", lateout(reg) result, in(reg) lhs, in(reg) rhs, options(pure, nomem, nostack)) }
    result
}

/// Floating-point add single-precision.
#[inline]
#[target_feature(enable = "f")]
pub fn fadd_s(lhs: f32, rhs: f32) -> f32 {
    let result;
    unsafe { asm!("fadd.s {}, {}, {}", lateout(freg) result, in(freg) lhs, in(freg) rhs, options(pure, nomem, nostack)) };
    result
}

/// Floating-point add double-precision.
#[inline]
#[target_feature(enable = "d")]
pub fn fadd_d(lhs: f64, rhs: f64) -> f64 {
    let result;
    unsafe { asm!("fadd.d {}, {}, {}", lateout(freg) result, in(freg) lhs, in(freg) rhs, options(pure, nomem, nostack)) };
    result
}

/// Shift left by 1 and add.
#[inline]
#[target_feature(enable = "zba")]
pub fn sh1add(lhs: usize, rhs: usize) -> usize {
    let result;
    unsafe { asm!("sh1add {}, {}, {}", lateout(reg) result, in(reg) lhs, in(reg) rhs, options(pure, nomem, nostack)) };
    result
}
