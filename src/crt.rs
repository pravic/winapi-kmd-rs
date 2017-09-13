//! C runtime library.
//!
//! Functions imported from `ntoskrnl.exe`.

extern "C"
{
	pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
	pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;

	pub fn strlen(s: *const u8) -> usize;
	pub fn strcmp(s1: *const u8, s2: *const u8) -> i32;
	pub fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8;
	pub fn strcat(dest: *mut u8, src: *const u8) -> *mut u8;
	pub fn strncpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;

	pub fn wcslen(s: *const u16) -> usize;
	pub fn wcscpy(dest: *mut u16, src: *const u16) -> *mut u16;
	pub fn wcsncpy(dest: *mut u16, src: *const u16, n: usize) -> *mut u16;
}


#[no_mangle]
#[allow(non_upper_case_globals)]
#[cfg(target_arch="x86")]
#[doc(hidden)]
pub static __security_cookie: usize = 0xBB40E64E;

#[no_mangle]
#[allow(non_upper_case_globals)]
#[cfg(target_arch="x86_64")]
#[doc(hidden)]
pub static __security_cookie: usize = 0x00002B992DDFA232;


#[doc(hidden)]
pub mod rust_intrinsics
{
	#![allow(unused_variables)]

	// Idk why, but linker cannot find `_memcmp` for llvm intrinsics. So lets make forward one.
	#[no_mangle]
	pub unsafe fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
		return ::crt::memcmp(s1, s2, n);
	}

	// ported from compiler-rt
	// pub fn __muldi3(a: u64, b: u64) -> u64 {
	// 	unimplemented!();
	// }

	#[no_mangle]
	pub fn __mulodi4(a: i64, b: i64, overflow: &mut i32) -> i64 {
		unimplemented!();
	}

	#[no_mangle]
	pub extern "C" fn __multi3(a: i128, b: i128) -> i128 {
		unimplemented!();
	}

	#[no_mangle]
	pub extern fn __muloti4(a: i128, b: i128, oflow: &mut i32) -> i128 {
		unimplemented!();
	}

	#[no_mangle]
	pub extern "C" fn __udivti3(n: u128, d: u128) -> u128 {
		unimplemented!();
	}

	#[no_mangle]
	pub extern "C" fn __umodti3(n: u128, d: u128) -> u128 {
		unimplemented!();
	}
}
