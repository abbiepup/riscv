use core::arch::asm;

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
