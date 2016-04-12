//! NT Status codes.
#![allow(non_camel_case_types)]
#![allow(overflowing_literals)]

/// NT Status type.
pub type NTSTATUS = Status;

/// A specialized `Result` type for NT operations.
pub type Result<T> = ::core::result::Result<T, Status>;


/// NT Status code.
#[repr(C)]
#[derive(Clone, Copy)]
pub enum Status {
	success = 0,
	unsuccessful = 0xC0000001,
}

impl Status {
	/// Evaluates to `true` if the `Status` is a success type (`0..0x3FFFFFFF`)
	/// or an informational type (`0x40000000..0x7FFFFFFF`).
	pub fn is_ok(&self) -> bool {
		(*self as i32) >= 0
	}
	/// Status is a warning or error type.
	pub fn is_err(&self) -> bool {
		(*self as i32) < 0
	}
	/// Status is a success type.
	pub fn is_success(&self) -> bool {
		let c = *self as u32;
		c > 0 && c < 0x3FFF_FFFF
	}
	/// Status is a information type.
	pub fn is_information(&self) -> bool {
		let c = *self as u32;
		c > 0x4000_0000 && c < 0x7FFF_FFFF
	}
	/// Status is a warning type.
	pub fn is_warning(&self) -> bool {
		let c = *self as u32;
		c > 0x8000_0000 && c < 0xBFFF_FFFF
	}
	/// Status is a error type.
	pub fn is_error(&self) -> bool {
		let c = *self as u32;
		c > 0xC000_0000 && c < 0xFFFF_FFFF
	}
}

/// Convert `Status` to `Result<()>`.
pub fn check(st: Status) -> Result<()> {
	if st.is_err() {
		Err(st)
	} else {
		Ok(())
	}
}
