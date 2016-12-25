use win32;
use win32_types::*;

pub struct Module {
    pub handle: usize,
}

impl Module {
    pub fn new(file_name: &[u8]) -> Option<Module> {

        let handle = win32::load_library(file_name) as usize;

        if handle == 0 {
            None
        } else {
            Some(Module { handle: handle })
        }
    }

    pub fn get_proc_address(&self, proc_index: isize) -> Proc {

        win32::get_proc_address(self.handle as ModuleHandle, proc_index)
    }
}
