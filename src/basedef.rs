//! Kernel-Mode Types.

use ::KIRQL;


// Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help enable
// more optimization opportunities around it recognizing things like
// malloc/free.
#[repr(u8)]
#[doc(hidden)]
pub enum km_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type VOID = km_void;
pub type PVOID = *mut VOID;
pub type PCVOID = *const VOID;


pub type PETHREAD = PVOID;
pub type PIO_APC_ROUTINE = Option<extern "system" fn (ApcContext: PCVOID, IoStatusBlock: *const IO_STATUS_BLOCK, Reserved: u32)>;


extern "system"
{
	pub fn KeGetCurrentIrql() -> KIRQL;
	pub fn KeRaiseIrqlToDpcLevel() -> KIRQL;
	pub fn KfLowerIrql(NewIrql: KIRQL) -> KIRQL;
	pub fn KfRaiseIrql(NewIrql: KIRQL) -> KIRQL;
}


/// Doubly linked list structure.
#[repr(C)]
pub struct LIST_ENTRY
{
	pub next: *mut LIST_ENTRY,
	pub prev: *mut LIST_ENTRY,
}

/// Spin Lock.
#[repr(C)]
pub struct KSPIN_LOCK
{
	pub lock: usize,
}

/// Common dispatcher object header.
#[repr(C)]
pub struct DISPATCHER_HEADER
{
	pub Type: u8,
	pub Absolute: u8,
	pub Size: u8,
	pub Inserted: u8,
	pub SignalState: i32,
	pub WaitListHead: LIST_ENTRY,
}

/// An I/O status block.
#[repr(C)]
pub struct IO_STATUS_BLOCK
{
	pub Status: ::NTSTATUS,
	pub Information: usize,
}

pub type PIO_STATUS_BLOCK = *mut IO_STATUS_BLOCK;


/// Processor modes.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum KPROCESSOR_MODE
{
	KernelMode,
	UserMode,
}

/// I/O Request priority.
pub mod IO_PRIORITY {
	/// I/O Request priority type.
	pub type KPRIORITY_BOOST = u8;

	pub const IO_NO_INCREMENT: KPRIORITY_BOOST = 0;
	pub const IO_DISK_INCREMENT: KPRIORITY_BOOST = 1;
	pub const EVENT_INCREMENT: KPRIORITY_BOOST = 1;
}
