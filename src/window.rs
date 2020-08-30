use std::mem;
use std::ptr::null_mut;
use winapi::um::winuser::*;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleA;

pub struct Window {
    handle: HWND,
}

impl Window {
    /// Create a new window
    pub fn new(name: &str) -> Self {
        let name = name.as_ptr() as *const i8;

        unsafe {
            let hinstance = GetModuleHandleA(null_mut());

            // Register our window class
            let wnd_class = WNDCLASSA {
                style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(DefWindowProcA),
                hInstance: hinstance,
                lpszClassName: name,
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: null_mut(),
                hCursor: null_mut(),
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
            };
            RegisterClassA(&wnd_class);

            // Create the actual window
            let handle = CreateWindowExA(
                0,
                name,
                name,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                null_mut(),
                null_mut(),
                hinstance,
                null_mut(),
            );

            return Window { handle: handle };
        }
    }

    /// Run the main loop of the window
    pub fn main_loop(&self) {
        loop {
            unsafe {
                let mut message = mem::MaybeUninit::<MSG>::uninit().assume_init();
                    if GetMessageA(&mut message as *mut MSG, self.handle, 0, 0) > 0 {
                        TranslateMessage(&message as *const MSG);
                        DispatchMessageA(&message as *const MSG);
                    }
            }
        }
    }

    /// Display an image on this window
    pub fn display(&self, path: String) {
        println!("displaying image: {}", path);
    }

    /// Destroy this window
    pub fn destroy(&self) {
        println!("destroying window...");
    }
}