use core::num::NonZero;

#[repr(transparent)]
pub struct Virtual(NonZero<usize>);

impl Virtual {
    #[inline]
    pub const fn new(address: usize) -> Option<Self> {
        match NonZero::new(address) {
            Some(address) => Some(Self(address)),
            None => None,
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(address: usize) -> Self {
        // SAFETY: The caller guarantees that `addr` is non-zero.
        Self(unsafe { NonZero::new_unchecked(address) })
    }

    #[inline]
    pub const fn get(self) -> usize {
        self.0.get()
    }
}
