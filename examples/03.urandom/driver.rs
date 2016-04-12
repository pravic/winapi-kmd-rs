#![feature(collections)]

#![no_std]
#![allow(bad_style)]

#[macro_use] extern crate km;
#[macro_use] extern crate collections;

use km::*;
use core::mem;
use core::ptr;

// Helper for converting b"string" to UNICODE_STRING
fn a2u(s: &[u8]) -> UnicodeString {
	let a = AnsiString::from(s);
	let mut u = UnicodeString::default();
	unsafe { RtlAnsiStringToUnicodeString(&mut u, &a, true) };
	return u;
}

// Our per-device settings payload
struct DEVICE_PARAMS
{
	pub name: UNICODE_STRING,
	pub link: UNICODE_STRING,
}

impl Drop for DEVICE_PARAMS
{
	fn drop(&mut self) {
		unsafe {
			RtlFreeUnicodeString(&mut self.name);
			RtlFreeUnicodeString(&mut self.link);
		}
	}
}


#[no_mangle]
pub extern "system" fn DriverEntry(driver: &mut km::DRIVER_OBJECT, _path: &km::string::UnicodeString) -> NTSTATUS
{
	KdPrint!("[rs] hello, rust!\n");
	KdPrint!("[rs] we are DriverObject at 0x%p, sizeof 0x%X (expected size 0xA8 or 0x150)\n",
		driver as *mut km::DRIVER_OBJECT, mem::size_of::<km::DRIVER_OBJECT>());

	driver.DriverUnload = Some(DriverUnload);

	KdPrint!("[rs] make params\n");
	let params = DEVICE_PARAMS { name: a2u(b"\\Device\\RandomUDev\0"), link: a2u(b"\\??\\urandom\0"), };

	// create device
	KdPrint!("[rs] create device `%ws` (%d bytes len)\n", params.name.Buffer, params.name.Length as u32);
	let mut device: *mut DEVICE_OBJECT = ptr::null_mut();
	check_unsafe!(IoCreateDevice(driver, mem::size_of::<DEVICE_PARAMS>() as u32, &params.name, 34, 0, false, &mut device));

	// store out custom params to DeviceExtension allocated memory
	KdPrint!("[rs] store params at device\n");
	let device = unsafe { &mut *device };
	let pparams = device.DeviceExtension as *mut DEVICE_PARAMS;
	let params = unsafe {
		ptr::write(pparams, params);
		&*pparams
	};

	// create symlink
	KdPrint!("[rs] create symlink `%ws`\n", params.link.Buffer);
	let st = unsafe { IoCreateSymbolicLink(&params.link, &params.name) };
	if st.is_err() {
		DriverUnload(driver);
		return st;
	}

	// setup I/O processing handlers
	use km::irp::IRP_MJ;
	driver.MajorFunction[IRP_MJ::CREATE as usize] = Some(DispatchCreateClose);
	driver.MajorFunction[IRP_MJ::CLOSE as usize] = Some(DispatchCreateClose);
	driver.MajorFunction[IRP_MJ::READ as usize] = Some(DispatchRead);
	device.Flags |= DEVICE_FLAGS::DO_BUFFERED_IO as u32;

	KdPrint!("[rs] loaded.\n");
	return Status::success;
}

extern "system" fn DriverUnload(driver: &mut km::DRIVER_OBJECT)
{
	KdPrint!("[rs] unload:\n");
	unsafe {
		// for each created device (driver.DeviceObject->NextDevice linked list)
		// delete symbolic link and delete device itself
		let mut pdevice = driver.DeviceObject;
		while !pdevice.is_null() {
			KdPrint_u!("[rs] free device\n");
			let device = &mut *pdevice;
			if !device.DeviceExtension.is_null() {
				KdPrint_u!("[rs] drop params\n");
				let params = &mut *(device.DeviceExtension as *mut DEVICE_PARAMS);
				IoDeleteSymbolicLink(&params.link);
				drop(params);
			}
			KdPrint_u!("[rs] delete device\n");
			pdevice = device.NextDevice;
			IoDeleteDevice(device);
		}
	}
	KdPrint!("[rs] unloaded.\n");
}

extern "system" fn DispatchCreateClose(_device: &mut DEVICE_OBJECT, irp: &mut IRP) -> NTSTATUS {
	KdPrint!("[rs] dispatch create/close \n");
	irp.IoStatus.Information = 0;
	return irp.complete_request(Status::success);
}

extern "system" fn DispatchRead(device: &mut DEVICE_OBJECT, irp: &mut IRP) -> NTSTATUS
{
	KdPrint!("[rs] dispatch read\n");
	if (device.Flags & DEVICE_FLAGS::DO_BUFFERED_IO as u32) == 0 {
		KdPrint!("[rs] error: nonbuffered io!\n");
		return irp.complete_request(Status::unsuccessful);
	}

	// process IRP request
	let size =
	{
		let io = irp.get_current_stack_location();
		let args = io.ParametersRead();
		args.Length as usize
	};
	KdPrint!("[rs] read size %d\n", size as i32);

	if size == 0 {
		KdPrint!("[rs] error: empty buffer!\n");
		return irp.complete_request(Status::unsuccessful);
	}

	KdPrint!("[rs] generate random\n");
	let buf = irp.SystemBuffer;					// AssociatedIrp.SystemBuffer union.
	let r = GenerateRandom(buf, size);
	let st = if let Ok(size) = r {
		irp.IoStatus.Information = size;
		Status::success
	} else {
		r.err().unwrap()
	};
	return irp.complete_request(st);
}

// RtlRandom: Random number generator based on MacLaren and Marsaglia.
// RtlRandomEx is twice as fast and produces better random numbers
// since the period of the random numbers generated is comparatively higher.
fn GenerateRandom(buffer: PVOID, size: usize) -> Result<usize> {
	let mut seed = km::time::QueryTickCount() as u32;
	let data = buffer as *mut u32;
	let dwords = size / 4;
	let tail = size % 4;
	KdPrint!("[rs] generate random for %d bytes as %d words and %d padding\n", size as u32, dwords as u32, tail as u32);
	unsafe {
		let mut i = 0;
		while i < dwords {
			let word = km::rtl::RtlRandomEx(&mut seed);
			*data.offset(i as isize) = word;
			i += 1;
		}
		if tail != 0 {
			let word = km::rtl::RtlRandomEx(&mut seed);
			km::crt::memcpy(data.offset(dwords as isize) as *mut u8, &word as *const u32 as *const u8, tail);
		}
	}
	KdPrint!("[rs] generate complete\n");
	return Ok(size);
}
