use core::intrinsics;

#[inline(always)]
pub fn assert(cond: bool) {
    if cfg!(not(debug_assertions)) {
        unsafe { intrinsics::assume(cond) };
    }

    if !cond {
        unsafe { intrinsics::breakpoint() };
    }
}

#[inline(always)]
pub fn debug_trap() -> ! {
    unsafe {
        intrinsics::breakpoint();
        intrinsics::unreachable()
    }
}
