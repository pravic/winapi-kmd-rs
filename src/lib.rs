//! Windows Kernel Mode library.

#![feature(lang_items)]
#![feature(on_unimplemented, fundamental)]
#![feature(no_core)]

#![no_std]
#![allow(bad_style)]


#[macro_use] pub mod macros;

mod lang;

pub mod status;

pub mod basedef;
pub mod crt;
pub mod debug;
pub mod device_object;
pub mod dpc;
pub mod driver_object;
pub mod event;
pub mod file_object;
pub mod irp;
pub mod irql;
pub mod object;
pub mod pool;
pub mod rtl;
pub mod string;
pub mod time;

#[doc(hidden)]
pub use irql::KIRQL;

#[doc(hidden)]
pub use status::*;

#[doc(hidden)]
pub use debug::DbgPrint;

#[doc(hidden)]
pub use string::*;

#[doc(hidden)]
pub use driver_object::*;

#[doc(hidden)]
pub use device_object::*;

#[doc(hidden)]
pub use irp::IRP;

#[doc(hidden)]
pub use basedef::{PVOID};
