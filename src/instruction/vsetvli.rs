use core::arch::asm;
use core::marker::ConstParamTy;

#[inline]
#[target_feature(enable = "v")]
pub fn vsetvli<S, const L: LMUL, const TA: bool, const MA: bool>(vl: usize) -> usize
where
    S: Sew,
{
    const fn vtype(sew: u8, lmul: LMUL, ta: bool, ma: bool) -> u8 {
        ((ma as u8) << 7) | ((ta as u8) << 6) | ((lmul as u8) << 3) | sew
    }

    let result;
    unsafe { asm!(".insn i 0x57, 7, {}, {}, {}", out(reg) result, in(reg) vl, const vtype(S::SEW, L, TA, MA)) };
    result
}

pub trait Sew: sealed::Sealed {
    const SEW: u8;
}

impl Sew for u8 {
    const SEW: u8 = 0;
}

impl Sew for u16 {
    const SEW: u8 = 1;
}

impl Sew for u32 {
    const SEW: u8 = 2;
}

impl Sew for u64 {
    const SEW: u8 = 3;
}

/// Vector Register Group Multiplier.
#[derive(ConstParamTy, PartialEq, Eq)]
pub enum LMUL {
    M1 = 0b000,
    M2 = 0b001,
    M4 = 0b010,
    M8 = 0b011,
    MF2 = 0b100,
    MF4 = 0b101,
    MF8 = 0b110,
}

mod sealed {
    pub trait Sealed {}
}

impl sealed::Sealed for u8 {}
impl sealed::Sealed for u16 {}
impl sealed::Sealed for u32 {}
impl sealed::Sealed for u64 {}
