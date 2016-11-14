use core::ptr;
use win32;
use win32_types::*;

pub struct Module {
    pub handle: ModuleHandle,
}

impl Module {
    pub fn new(file_name: &[u8]) -> Option<Module> {

        let handle = win32::load_library(file_name);

        if handle == ptr::null_mut() {
            None
        } else {
            Some(Module { handle: handle })
        }
    }

    pub fn get_proc_address(&self, proc_index: isize) -> Proc {

        win32::get_proc_address(self.handle, proc_index)
    }
}
