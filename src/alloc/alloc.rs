//! Kernel Mode Allocator

#![crate_name = "alloc_system"]
#![crate_type = "rlib"]

// Allocators are not allowed to depend on the standard library which in turn
// requires an allocator in order to avoid circular dependencies. This crate,
// however, can use all of libcore.
#![no_std]

// The compiler needs to be instructed that this crate is an allocator in order
// to realize that when this is linked in another allocator like jemalloc should
// not be linked in
#![feature(global_allocator)]
#![feature(default_lib_allocator)]
//#![allocator]



mod pool;

// Listed below are the five allocation functions currently required by custom
// allocators. Their signatures and symbol names are not currently typechecked
// by the compiler, but this is a future extension and are required to match
// what is found below.
//
// Note that the standard `malloc` and `realloc` functions do not provide a way
// to communicate alignment so this implementation would need to be improved
// with respect to alignment in that aspect.

const KMRS_TAG: u32 = 0x4B4D5253; // 'KMRS'
use pool::{ExAllocatePoolWithTag, ExFreePoolWithTag, POOL_TYPE};

extern "C"
{
	// from ntoskrnl
	pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
}

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, _align: usize) -> *mut u8
{
	unsafe { ExAllocatePoolWithTag(POOL_TYPE::PagedPool, size, KMRS_TAG) }
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize)
{
	unsafe { ExFreePoolWithTag(ptr, KMRS_TAG) };
}

#[no_mangle]
pub extern "C" fn __rust_reallocate(old: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8
{
	unsafe {
		// http://en.cppreference.com/w/c/memory/realloc
		let minsize = if size < old_size { size } else { old_size };
		let new = __rust_allocate(size, align);
		if new.is_null() {
			return new;
		}
		if !old.is_null() && old_size > 0 {
			memcpy(new, old, minsize);
			__rust_deallocate(old, old_size, align);
		}
		return new;
	}
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize) -> usize
{
	old_size // this api is not supported
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize
{
	size
}
