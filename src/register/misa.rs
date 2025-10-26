use crate::XLEN;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

#[must_use]
#[inline(always)]
pub fn read() -> (XLEN, Extension) {
    let misa: usize;
    unsafe { core::arch::asm!("csrr {}, misa", out(reg) misa, options(nomem, nostack)) };

    let extension = Extension(misa & 0x3FFFFFF);
    let xlen = XLEN::try_from((misa >> (usize::BITS as usize - 2)) & 0b11).unwrap_or_default();

    (xlen, extension)
}

impl Default for XLEN {
    fn default() -> Self {
        #[cfg(target_pointer_width = "32")]
        {
            Self::X32
        }
        #[cfg(target_pointer_width = "64")]
        {
            Self::X64
        }
        #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
        {
            Self::X128
        }
    }
}



#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extension(usize);

impl Extension {
    /// Atomic extension.
    pub const A: Self = Self(1 << 0);
    /// Compressed extension.
    pub const C: Self = Self(1 << 2);
    /// Double-precision floating-point extension.
    pub const D: Self = Self(1 << 3);
    /// RV32E base ISA.
    pub const E: Self = Self(1 << 4);
    /// Single-precision floating-point extension.
    pub const F: Self = Self(1 << 5);
    /// Hypervisor extension.
    pub const H: Self = Self(1 << 7);
    /// RV32I/64I/128I base ISA.
    pub const I: Self = Self(1 << 8);
    /// Integer Multiply/Divide extension.
    pub const M: Self = Self(1 << 12);
    /// Quad-precision floating-point extension.
    pub const Q: Self = Self(1 << 16);
    /// Supervisor mode implemented.
    pub const S: Self = Self(1 << 18);
    /// User mode implemented.
    pub const U: Self = Self(1 << 20);
}

impl TryFrom<char> for Extension {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::A,
            'C' => Self::C,
            'D' => Self::D,
            'E' => Self::E,
            'F' => Self::F,
            'H' => Self::H,
            'I' => Self::I,
            'M' => Self::M,
            'Q' => Self::Q,
            'S' => Self::S,
            'U' => Self::U,
            _ => return Err(()),
        })
    }
}

impl BitOr for Extension {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Extension {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Extension {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Extension {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for Extension {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
