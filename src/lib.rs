#![no_std]
#![feature(abi_riscv_interrupt)]
#![feature(adt_const_params)]
#![feature(decl_macro)]
#![feature(doc_cfg)]
#![feature(riscv_target_feature)]

// #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
// compile_error!("This crate only supports RISC-V 32/64");

mod address;
mod instruction;
mod register;

pub use address::*;
pub use instruction::*;
pub use register::*;

use core::marker::ConstParamTy;

/// Vector register group multiplier.
#[repr(u8)]
#[non_exhaustive]
#[derive(ConstParamTy, PartialEq, Eq)]
pub enum LMUL {
    /// 1× multiplier
    M1 = 0b000,
    /// 2× multiplier.
    M2 = 0b001,
    /// 4× multiplier.  
    M4 = 0b010,
    /// 8× multiplier. 
    M8 = 0b011,
    /// ½× fractional multiplier. 
    MF2 = 0b100,
    /// ¼× fractional multiplier.  
    MF4 = 0b101,
    /// ⅛× fractional multiplier.
    MF8 = 0b110,
}

impl Into<f32> for LMUL {
    #[inline]
    fn into(self) -> f32 {
        match self {
            Self::M1 => 1.0,
            Self::M2 => 2.0,
            Self::M4 => 4.0,
            Self::M8 => 8.0,
            Self::MF2 => 0.5,
            Self::MF4 => 0.25,
            Self::MF8 => 0.125,
        }
    }
}

impl Into<f64> for LMUL {
    #[inline]
    fn into(self) -> f64 {
        match self {
            Self::M1 => 1.0,
            Self::M2 => 2.0,
            Self::M4 => 4.0,
            Self::M8 => 8.0,
            Self::MF2 => 0.5,
            Self::MF4 => 0.25,
            Self::MF8 => 0.125,
        }
    }
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XLEN {
    X32 = 1,
    X64 = 2,
    X128 = 3,
}

impl TryFrom<usize> for XLEN {
    type Error = ();

    fn try_from(xlen: usize) -> Result<Self, Self::Error> {
        match xlen {
            1 => Ok(Self::X32),
            2 => Ok(Self::X64),
            3 => Ok(Self::X128),
            _ => Err(()),
        }
    }
}

/// Selected element width.
#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SEW {
    /// Selected [`u8`] element width.
    E8 = 0,
    /// Selected [`u16`] element width.
    E16 = 1,
    /// Selected [`u32`] element width.
    E32 = 2,
    /// Selected [`u64`] element width.
    E64 = 3,
    /// Selected [`u128`] element width.
    E128 = 4,
}
