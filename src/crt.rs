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
	// Idk why, but linker cannot find `_memcmp` for llvm intrinsics. So lets make forward one.
	#[no_mangle]
	pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
		return ::crt::memcmp(s1, s2, n);
	}

	// ported from compiler-rt
	#[no_mangle]
	pub unsafe extern fn __mulodi4(a: i64, b: i64, overflow: &mut i32) -> i64 {
		const N: i32 = 64;
		const MIN: i64 = 1 << (N-1);
		const MAX: i64 = !MIN;
		*overflow = 0;

		let result = a * b;
		if a == MIN {
			if b != 0 && b != 1 {
				*overflow = 1;
			}
			return result;
		}

		if b == MIN {
			if a != 0 && a != 1 {
				*overflow = 1;
			}
			return result;
		}

		let sa = a >> (N-1);
		let sb = b >> (N-1);
		let abs_a = (a ^ sa) - sa;
		let abs_b = (b ^ sb) - sb;

		if abs_a < 2 || abs_b < 2 {
			return result;
		}
		if sa == sb {
			if abs_a > MAX / abs_b {
				*overflow = 1;
			}
		} else {
			if abs_a > MIN / -abs_b {
				*overflow = 1;
			}
		}
		return result;
	}
}
