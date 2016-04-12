#![crate_type = "dylib"]
#![no_std]
#![allow(bad_style)]

#[macro_use] extern crate km;

use core::mem;
use km::*;

#[no_mangle]
pub extern "system" fn DriverEntry(driver: &mut km::DRIVER_OBJECT, _path: *const km::string::UnicodeString) -> Status {

	KdPrint!("[rs] hello, rust!\n");
	KdPrint!("[rs] we are DriverObject at 0x%p, sizeof %d\n", driver as *mut km::DRIVER_OBJECT , mem::size_of::<km::DRIVER_OBJECT>());

	driver.DriverUnload = Some(DriverUnload);
	return Status::success;
}

extern "system" fn DriverUnload(_obj: &mut km::DRIVER_OBJECT) {
	KdPrint!("[rs] unload.\n");
}
