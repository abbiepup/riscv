#![no_std]
#![feature(doc_cfg)]
#![feature(abi_riscv_interrupt)]
#![feature(riscv_target_feature)]

// #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
// compile_error!("This crate only supports RISC-V 32/64");

pub mod address;
pub mod instruction;
pub mod register;
