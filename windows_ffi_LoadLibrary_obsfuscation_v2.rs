#![windows_subsystem = "windows"]

use std::ffi::CString;
use std::ptr;

#[link(name = "kernel32")]
extern "stdcall" {
    pub fn LoadLibraryA(lpFileName: *const i8) -> *const usize;
    pub fn GetProcAddress(hModule: *const usize, lpProcName: *const i8) -> *const usize;
}

type FnMessageBox = extern "stdcall" fn(*const i8, *const i8, *const i8, u32) -> i32;
unsafe fn get_message_box() -> FnMessageBox {
    let mut message_box_api = String::from("MQXXagQBoxA");
    message_box_api.replace_range(2..4, "ss");
    message_box_api.replace_range(1..2, "e");
    message_box_api.replace_range(6..7, "e");
    let c_message_box_api =  CString::new(message_box_api).expect("");
    let c_user32 = CString::new("user32").expect("");

    let module = LoadLibraryA(c_user32.as_ptr());
    let h_message_box = GetProcAddress(module, c_message_box_api.as_ptr());
    let message_box = std::mem::transmute::<*const usize, FnMessageBox>(h_message_box);

    message_box
} 


fn main() {
    let c_msg = CString::new("What a time to be alive").expect("");
    let c_title = CString::new("GetProcAddress in Rust").expect("");

    unsafe {        
        let message_box = get_message_box();
        message_box(ptr::null(), c_msg.as_ptr(), c_title.as_ptr(), 0);
    }

}


