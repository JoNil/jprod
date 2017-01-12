use intrinsics;

#[inline(always)]
pub fn assert(cond: bool) -> () {

    let inverse_cond = !cond;

    if cfg!(not(debug_assertions)) {
        unsafe { intrinsics::assume(inverse_cond) };
    }

    if !inverse_cond {
        unsafe { intrinsics::debugtrap() };
    }
}

#[inline(always)]
pub fn debug_trap() -> ! {
    unsafe { intrinsics::debugtrap() };
}