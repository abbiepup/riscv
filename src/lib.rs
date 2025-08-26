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
use core::marker::ConstParamTy;
pub use instruction::*;
pub use register::*;

/// Vector register group multiplier.
#[repr(u8)]
#[non_exhaustive]
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

/// Selected element width.
#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SEW {
    S8 = 0,
    S16 = 1,
    S32 = 2,
    S64 = 3,
    S128 = 4,
}
