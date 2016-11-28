#[macro_export]
macro_rules! load_proc {
    ($module:expr, $name:expr) => {
        {
            use core;

            let procedure = $module.get_proc_address($name);
            if procedure == core::ptr::null_mut() {
                win32::debug_break();
            }
            #[allow(unused_unsafe)]
            unsafe { core::intrinsics::transmute(procedure) }
        }
    }
}
