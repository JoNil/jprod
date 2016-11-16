use module::Module;
use win32_types::*;

static mut API: Option<Api> = None;

#[allow(non_snake_case)]
struct Api {
    #[allow(dead_code)]
    gdi32: Module,

    ChoosePixelFormat: unsafe extern "system" fn(dc: DcHandle,
                                                 descriptor: *const PixelFormatDescriptor)
                                                 -> i32,

    DescribePixelFormat: unsafe extern "system" fn(dc: DcHandle,
                                                   pixel_format: i32,
                                                   bytes: u32,
                                                   descriptor: *mut PixelFormatDescriptor)
                                                   -> i32,

    SetPixelFormat: unsafe extern "system" fn(dc: DcHandle,
                                              pixel_format: i32,
                                              descriptor: *const PixelFormatDescriptor)
                                              -> i32,

    SwapBuffers: unsafe extern "system" fn(dc: DcHandle) -> i32,
}

#[inline]
fn api() -> &'static Api {
    unsafe {
        if let Some(ref api) = API {
            api
        } else {
            panic!();
        }
    }
}

pub fn init() {

    if let Some(gdi32) = Module::new(b"Gdi32.dll\0") {

        unsafe {
            API = Some(Api {
                ChoosePixelFormat: load_proc!(gdi32, 999 + 45),

                DescribePixelFormat: load_proc!(gdi32, 999 + 360),

                SetPixelFormat: load_proc!(gdi32, 999 + 1491),

                SwapBuffers: load_proc!(gdi32, 999 + 1528),

                gdi32: gdi32,
            })
        }
    } else {
        panic!();
    }
}

pub fn choose_pixel_format(dc: DcHandle, descriptor: &PixelFormatDescriptor) -> i32 {

    unsafe { (api().ChoosePixelFormat)(dc, descriptor as *const PixelFormatDescriptor) }
}

pub fn describe_pixel_format(dc: DcHandle,
                             pixel_format: i32,
                             bytes: u32,
                             descriptor: &mut PixelFormatDescriptor)
                             -> i32 {

    unsafe {
        (api().DescribePixelFormat)(dc,
                                    pixel_format,
                                    bytes,
                                    descriptor as *mut PixelFormatDescriptor)
    }
}

pub fn set_pixel_format(dc: DcHandle,
                        pixel_format: i32,
                        descriptor: *const PixelFormatDescriptor)
                        -> i32 {

    unsafe { (api().SetPixelFormat)(dc, pixel_format, descriptor as *const PixelFormatDescriptor) }
}

pub fn swap_buffers(dc: DcHandle) -> bool {

    unsafe { (api().SwapBuffers)(dc) != 0 }
}