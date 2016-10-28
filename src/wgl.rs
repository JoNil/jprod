use win32;

#[allow(non_snake_case)]
pub struct Api {
    
}

impl Api {

     pub fn new() -> Api {
        
        let user32 = win32::load_library_a(b"user32.dll\0");
        if user32 == ptr::null_mut() {
            panic!();
        }

        Api {
            user32: user32,
            
            MessageBoxA: load_proc!(user32,  b"MessageBoxA\0"),

            RegisterClassA: load_proc!(user32, b"RegisterClassA\0"),
            CreateWindowExA: load_proc!(user32, b"CreateWindowExA\0"),

            GetMessageA: load_proc!(user32, b"GetMessageA\0"),
            TranslateMessage: load_proc!(user32, b"TranslateMessage\0"),
            DispatchMessageA: load_proc!(user32, b"DispatchMessageA\0"),
            DefWindowProcA: load_proc!(user32, b"DefWindowProcA\0"),

            LoadCursorA: load_proc!(user32, b"LoadCursorA\0"),
        }
    }

    #[inline]
    pub fn create_context(w32: &win32::Api) {



    }
}