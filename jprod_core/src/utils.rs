#[inline(always)]
pub fn assert(cond: bool) {
    if !cond {
        #[allow(clippy::empty_loop)]
        loop {}
    }
}

#[inline(always)]
pub fn debug_trap() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}
