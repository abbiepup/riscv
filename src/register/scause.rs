#[inline]
#[must_use]
pub fn read() -> Cause {
    let scause: usize;
    unsafe { core::arch::asm!("csrr {}, scause", out(reg) scause, options(nomem, nostack)) };

    let is_interrupt = (scause >> (core::mem::size_of::<usize>() * 8 - 1)) != 0;
    let code = scause & !(1 << (core::mem::size_of::<usize>() * 8 - 1));

    if is_interrupt {
        unsafe { Cause::Interrupt(Interrupt::try_from(code).unwrap_unchecked()) }
    } else {
        unsafe { Cause::Exception(Exception::try_from(code).unwrap_unchecked()) }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    Exception(Exception),
    Interrupt(Interrupt),
}

#[repr(usize)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exception {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAddressMisaligned = 6,
    StoreAccessFault = 7,
    UserEnvironmentCall = 8,
    SupervisorEnvironmentCall = 9,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
}

impl TryFrom<usize> for Exception {
    type Error = ();

    fn try_from(exception: usize) -> Result<Self, Self::Error> {
        Ok(match exception {
            0 => Self::InstructionAddressMisaligned,
            1 => Self::InstructionAccessFault,
            2 => Self::IllegalInstruction,
            3 => Self::Breakpoint,
            4 => Self::LoadAddressMisaligned,
            5 => Self::LoadAccessFault,
            6 => Self::StoreAddressMisaligned,
            7 => Self::StoreAccessFault,
            8 => Self::UserEnvironmentCall,
            9 => Self::SupervisorEnvironmentCall,
            12 => Self::InstructionPageFault,
            13 => Self::LoadPageFault,
            15 => Self::StorePageFault,
            _ => return Err(()),
        })
    }
}

#[repr(usize)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {
    Software = 1,
    Timer = 5,
    External = 9,
}

impl TryFrom<usize> for Interrupt {
    type Error = ();

    fn try_from(interrupt: usize) -> Result<Self, Self::Error> {
        Ok(match interrupt {
            1 => Self::Software,
            5 => Self::Timer,
            9 => Self::External,
            _ => return Err(()),
        })
    }
}
