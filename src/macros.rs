//! Macros for Kernel-Mode drivers.

/// Macro to send a message to the kernel debugger.
///
/// # Example
///
/// ```no_run
/// KdPrint!("NTSTATUS is 0x%X\n", status);
/// ```
#[macro_export]
macro_rules! KdPrint {
	($msg:expr $(, $arg:expr)*) => { unsafe { $crate::debug::DbgPrint( concat!($msg, "\0").as_ptr() $(, $arg )* )} };
}

/// Macro to send a message to the kernel debugger for unsafe blocks.
///
/// Used in `unsafe {}` blocks.
#[macro_export]
macro_rules! KdPrint_u {
	($msg:expr $(, $arg:expr)*) => { $crate::debug::DbgPrint( concat!($msg, "\0").as_ptr() $(, $arg )* ) };
}

#[macro_export]
macro_rules! check_unsafe {
	($expr:expr) => {{
		let st: $crate::status::Status = unsafe { $expr };
		if st.is_err() {
			KdPrint!("[km] error: status 0x%X\n", st);
			return st;
		} else {
			st
		}
	}}
}
