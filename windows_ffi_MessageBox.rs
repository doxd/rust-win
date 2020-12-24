#![windows_subsystem = "windows"]

use std::ffi::CString;
use core::ptr;

  #[link(name = "user32")]
    extern "stdcall" {
        pub fn MessageBoxA(
            hWnd: *const i8,
            lpText: *const i8,
            lpCaption: *const i8,
            uType: u32
        ) -> i32;
    }


fn main() {
    let msg = CString::new("Hello from Rust").expect("");
    let title = CString::new("Holy smokes!").expect("");
    unsafe {
        MessageBoxA(
            ptr::null(),
            msg.as_ptr(),
            title.as_ptr(),
            0);
    }
}
