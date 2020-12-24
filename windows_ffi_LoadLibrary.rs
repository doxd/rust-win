use std::ptr;
use std::ffi::CString;

#[link(name = "kernel32")]
extern "stdcall" {
    pub fn LoadLibraryA(lpFileName: *const u8) -> *const usize;
    pub fn GetProcAddress(hModule: *const usize, lpProcName: *const u8) -> *const usize;
}

type FnMessageBox = extern "stdcall" fn(*const (), *const u8, *const u8, u32) -> i32;
const USER32_DLL: &'static [u8] = b"user32\0";
const MESSAGE_BOX: &'static [u8] = b"MessageBoxA\0";
const TITLE: &'static [u8] = b"GetProcAddress in Rust\0";

fn main() {
    let mut msg = String::new();

    unsafe {
        let module = LoadLibraryA(USER32_DLL.as_ptr() as *const u8);
        msg.push_str(&format!("module {}\n", module as usize));
        let h_message_box = GetProcAddress(module, MESSAGE_BOX.as_ptr() as *const u8);
        msg.push_str(&format!("MessageBoxA {}", h_message_box as usize));
        let message_box = std::mem::transmute::<*const usize, FnMessageBox>(h_message_box);
        let c_msg = CString::new(msg).expect("");
        message_box(
            ptr::null(),
            c_msg.as_ptr() as *const u8,
            TITLE.as_ptr() as *const u8,
            0,
        );
    }
}


