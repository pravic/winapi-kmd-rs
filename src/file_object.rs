//! File Object.

use ::device_object::PDEVICE_OBJECT;

pub type PFILE_OBJECT = *mut FILE_OBJECT;

/// The `FILE_OBJECT` structure is used by the system to represent a file object.
#[repr(C)]
pub struct FILE_OBJECT
{
	Type: u16,
	Size: u16,
	DeviceObject: PDEVICE_OBJECT,
	// ...
}
