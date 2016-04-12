//! NT Time routines.

/// System time is a count of 100-nanosecond intervals since January 1, 1601.
pub type SYSTEMTIME = i64;

extern "system"
{
	fn KeQuerySystemTime(CurrentTime: *mut SYSTEMTIME);
	fn KeQueryTickCount(TickCount: *mut i64);
	/// Converts a GMT system time value to the local system time for the current time zone.
	pub fn ExSystemTimeToLocalTime(SystemTime: *const SYSTEMTIME, LocalTime: *mut SYSTEMTIME);
}

/// Obtains the current system time.
pub fn QuerySystemTime() -> SYSTEMTIME {
	let mut t = 0i64;
	unsafe { KeQuerySystemTime(&mut t) };
	return t;
}

/// A count of the interval timer interrupts that have occurred since the system was booted.
pub fn QueryTickCount() -> i64 {
	let mut t = 0i64;
	unsafe { KeQueryTickCount(&mut t) };
	return t;
}
