use intrinsics;

pub fn debug_trap_if(cond: bool) -> () {

    let inverse_cond = !cond;

    if cfg!(not(debug_assertions)) {
        unsafe { intrinsics::assume(inverse_cond) };
    }

    if !inverse_cond {
        unsafe { intrinsics::debugtrap() };
    }
}

pub fn debug_trap() -> ! {
    unsafe { intrinsics::debugtrap() };
}