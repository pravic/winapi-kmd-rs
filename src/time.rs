//! NT Time routines.

use ::shared::{SYSTEMTIME};

#[cfg(target_arch = "x86_64")]
use ::shared::{KUSER_SHARED_DATA};


extern "system"
{
	// The following exports exists only on x86 kernels.
	// x64 drivers must use KUSER_SHARED_DATA to obtain these values.

	#[cfg(target_arch = "x86")]
	fn KeQuerySystemTime(CurrentTime: *mut SYSTEMTIME);
	#[cfg(target_arch = "x86")]
	fn KeQueryTickCount(TickCount: *mut i64);

	/// Converts a GMT system time value to the local system time for the current time zone.
	pub fn ExSystemTimeToLocalTime(SystemTime: *const SYSTEMTIME, LocalTime: *mut SYSTEMTIME);
}


/// Obtains the current system time.
#[cfg(target_arch = "x86")]
pub fn QuerySystemTime() -> SYSTEMTIME {
	let mut t = 0i64;
	unsafe { KeQuerySystemTime(&mut t) };
	return t;
}

/// Obtains the current system time.
#[cfg(target_arch = "x86_64")]
pub fn QuerySystemTime() -> SYSTEMTIME {
	let shared = KUSER_SHARED_DATA::get();
	SYSTEMTIME::from(shared.SystemTime)
}


/// A count of the interval timer interrupts that have occurred since the system was booted.
#[cfg(target_arch = "x86")]
pub fn QueryTickCount() -> i64 {
	let mut t = 0i64;
	unsafe { KeQueryTickCount(&mut t) };
	return t;
}


/// A count of the interval timer interrupts that have occurred since the system was booted.
#[cfg(target_arch = "x86_64")]
pub fn QueryTickCount() -> i64 {
	let shared = KUSER_SHARED_DATA::get();
	SYSTEMTIME::from(shared.TickCount)
}
