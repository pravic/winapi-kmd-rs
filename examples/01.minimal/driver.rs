#![crate_type = "dylib"]
#![no_std]
#![allow(bad_style)]

#[macro_use] extern crate km;

use core::mem;
use km::*;

#[no_mangle]
pub extern "system" fn DriverEntry(driver: *mut km::DRIVER_OBJECT, _path: *const km::string::UnicodeString) -> Status
{
	KdPrint!("[rs] hello, rust!\n");
	let cb = mem::size_of::<km::DRIVER_OBJECT>();
	KdPrint!("[rs] we are DriverObject at 0x%p, sizeof 0x%X (%d bytes), expected size 0xA8 or 0x150\n", driver, cb, cb);
	return Status::unsuccessful;	// return error to unload driver now
}
