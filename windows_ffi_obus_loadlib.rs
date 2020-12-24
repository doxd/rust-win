#![windows_subsystem = "windows"]

use std::{mem::transmute, ptr, ffi::CString};

#[link(name="kernel32")]
extern {
	pub fn LoadLibraryA(lpLibFileName: *const i8) -> *const i8;
	pub fn GetProcAddress(hModule: *const i8, lpProcName: *const i8) -> *const i8;
}

fn main() {
	let user32_str = CString::new("user32").expect("");
	let user32_handle = unsafe{ LoadLibraryA(user32_str.as_ptr()) };
	let message_box_str = CString::new(String::from("NfttbhfCpyB").bytes().map(|x| ((x-1) as char) ).collect::<String>()).expect("");
	let message_box: fn (*const i8, *const i8, *const i8, i32) = 
		unsafe{ transmute( GetProcAddress(user32_handle, message_box_str.as_ptr()) ) } ;
    message_box(ptr::null(), CString::new("This is the body").expect("").as_ptr(), 
    	CString::new("This is the title").expect("").as_ptr(), 0);
}

