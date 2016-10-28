#[macro_export]
macro_rules! load_proc {
    ($module:expr, $name:expr) => {
        {
            let procedure = $module.get_proc_address($name);
            if procedure == ptr::null_mut() {
                panic!();
            }
            unsafe { transmute(procedure) }
        }
    }
}