#![windows_subsystem = "windows"]

use std::ffi::CString;
use std::mem::transmute;
use std::ptr;

#[link(name = "kernel32")]
extern "stdcall" {
	pub fn LoadLibraryA(lpFileName: *const i8) -> *const i8;
	pub fn GetProcAddress(hModule: *const i8, lpProcName: *const i8) -> *const i8;
	pub fn CreateProcessA(lpApplicationName: *const i8, lpCommandLine: *const i8, lpProcessAttributes: *const i8, 
			lpThreadAttributes: *const i8, bInheritHandles: i32, dwCreationFlags: i32, lpEnvironment: *const i8,
			lpCurrentDirectory: *const i8, lpStartupInfo: *const i32, lpProcessInformation: *const i32) -> i32;
	pub fn GetLastError() -> i32;
}

#[link(name = "user32")]
extern "stdcall" {
	pub fn MessageBoxA(hWnd: *const i8, lpText: *const i8, lpCation: *const i8, uType: usize);
}

fn main() {
    // MessageBox
    let msg = CString::new("Banana").expect("");
    let title = CString::new("Message from Banana").expect("");    	
    unsafe { MessageBoxA(ptr::null(), msg.as_ptr(), title.as_ptr(), 0); }

    // CreateProcess and GetLastError
    let cmd = CString::new("c:\\windows\\notepad.exe c:\\users\\user\\desktop\\banana\\src\\main.rs").expect("");
    let startup_info: Vec<i32> = vec![0; 16];
    let process_info: Vec<i32> = vec![0; 16];
    let mut result_code: i32;
    unsafe { result_code = CreateProcessA(ptr::null(), cmd.as_ptr(), ptr::null(), ptr::null(), 
    	1, 0, ptr::null(), ptr::null(), startup_info.as_ptr(), process_info.as_ptr()); }
    println!("CreateProcessA result: {}", result_code);
    unsafe { result_code = GetLastError(); }
    println!("GetLastError result: {}", result_code);
    println!("lpStartupInfo:\t\t  {:?}\nlpProcessInformation: {:?}", startup_info, process_info);
    println!("PID of launched process is {}", process_info[4]);

    // LoadLibrary/GetProcAddress is used to load "Beep(dwFreq, dwDuration)" in kernel32, and std::mem::transmute converts it to a fn ptr
    let user32 = unsafe{ LoadLibraryA(CString::new("user32").expect("").as_ptr()) };
	let kernel32 = unsafe{ LoadLibraryA(CString::new("kernel32").expect("").as_ptr()) };
	// Interesting note: once loaded via LoadLibrary/GetProcAddress and transmuted, such functions don't need to be in unsafe blocks (?)
	let beep_name = String::from("BXXeXXeXXp").chars().filter(|x| *x != 'X').collect::<String>();
    let beep: extern "stdcall" fn(i32, i32) = unsafe {
    	transmute(GetProcAddress(kernel32, CString::new(beep_name).expect("").as_ptr()))
    };
    beep(1200, 1000);
    
    let mbox_name = String::from("Mes99sag99eB99oxA9").chars().filter(|x| *x != '9').collect::<String>();
    let message_box: extern "stdcall" fn(*const i8, *const i8, *const i8, usize) = 
    	unsafe { transmute(GetProcAddress(user32, CString::new(mbox_name).expect("").as_ptr())) };
    let msg_dyn = CString::new("Loaded MessageBoxA via LoadLibrary").expect("");
    message_box(ptr::null(), msg_dyn.as_ptr(), title.as_ptr(), 0); 
}


