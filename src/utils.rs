use core::mem;
use intrinsics;

pub fn debug_trap() -> ! {
    unsafe { intrinsics::debugtrap() };
}