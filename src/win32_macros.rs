#[macro_export]
macro_rules! load_proc {
    ($module:expr, $name:expr) => {
        {
            use core;

            let procedure = $module.get_proc_address($name as *const u8);
            if procedure == core::ptr::null_mut() {
                panic!();
            }
            #[allow(unused_unsafe)]
            unsafe { core::intrinsics::transmute(procedure) }
        }
    }
}