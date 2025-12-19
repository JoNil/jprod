#[inline(always)]
pub fn assert(cond: bool) {
    if cfg!(debug_assertions) && !cond {
        #[allow(clippy::empty_loop)]
        loop {}
    }
}

#[inline(always)]
pub fn debug_trap() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}
