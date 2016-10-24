use core::ptr;

pub enum Void {}

#[link_args = "kernel32.lib"]
extern "system" {
    fn OutputDebugStringA(output_string: *const u8);

    fn ExitProcess(exit_code: u32);

    //fn MessageBoxA(hWnd: *const Void, lpText: *const u8, lpCaption: *const u8, uType: u32) -> i32;
}

#[inline]
pub fn output_debug_string_a(string: &[u8])
{
    unsafe { OutputDebugStringA(&string[0]); }
}

#[inline]
pub fn exit_process(exit_code: u32)
{
    unsafe { ExitProcess(exit_code); }
}




/*pub fn message_box(text: &[u8], caption: &[u8], box_type: u32)
{
    unsafe {
        MessageBoxA(ptr::null(), &text[0], &caption[0], box_type);
    }
}*/