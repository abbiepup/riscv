// Read the supervisor trap vector base address register.
#[must_use]
#[inline(always)]
pub fn read() -> Result<Mode, Error> {
    let stvec: usize;
    unsafe { core::arch::asm!("csrr {}, stvec", out(reg) stvec, options(nomem, nostack)) };
    Mode::try_from(stvec)
}

/// Write to the supervisor trap vector base address register.
#[inline(always)]
pub unsafe fn write(mode: Mode) {
    let stvec: usize = mode.into();
    unsafe { core::arch::asm!("csrw stvec, {}", in(reg) stvec, options(nomem, nostack)) };
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Direct(extern "riscv-interrupt-s" fn()) = 0,
    Vectored(*const ()) = 1,
}

impl From<Mode> for usize {
    #[inline]
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Direct(handler) => (handler as usize) & !0b11 | 0,
            Mode::Vectored(vector) => ((vector as usize) & !0b11) | 1,
        }
    }
}

impl TryFrom<usize> for Mode {
    type Error = Error;

    #[inline]
    fn try_from(mode: usize) -> Result<Self, Self::Error> {
        match mode & 0b11 {
            0 => Ok(Mode::Direct(unsafe { core::mem::transmute((mode & !0b11) as *const ()) })),
            1 => Ok(Mode::Vectored((mode & !0b11) as *const ())),
            mode => Err(Error::Invalid(mode)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid or unimplemented stvec mode: {0}")]
    Invalid(usize),
}
