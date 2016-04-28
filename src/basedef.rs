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

pub type CHAR = i8;
pub type CCHAR = i8;
pub type USHORT = u16;
pub type CSHORT = i16;
pub type ULONG = u32;

pub type VOID = km_void;
pub type PVOID = *mut VOID;
pub type PCVOID = *const VOID;

pub type SIZE_T = usize;

pub type ULONG_PTR = usize;

pub type PEPROCESS = PVOID;
pub type PETHREAD = PVOID;
pub type PSECURITY_DESCRIPTOR = PVOID;

pub type PGUID = PVOID;
pub type PCGUID = PCVOID;

pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;

pub type PIO_APC_ROUTINE = Option<extern "system" fn (ApcContext: PCVOID, IoStatusBlock: *const IO_STATUS_BLOCK, Reserved: u32)>;


extern "system"
{
	pub fn KeGetCurrentIrql() -> KIRQL;
	pub fn KeRaiseIrqlToDpcLevel() -> KIRQL;
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
#[derive(Default)]
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
#[derive(Default, Clone, Copy)]
pub struct IO_STATUS_BLOCK
{
	/// Completion status.
	pub Status: ::NTSTATUS,
	/// Request-dependent value.
	pub Information: usize,
}

pub type PIO_STATUS_BLOCK = *mut IO_STATUS_BLOCK;

impl IO_STATUS_BLOCK {
	/// Return integer value for `Information` field.
	pub fn as_size(&self) -> usize {
		self.Information
	}

	/// Return the pointer of specified object type.
	pub fn as_ptr<T>(&self) -> *const T {
		unsafe { ::core::mem::transmute(self.Information) }
	}
}


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

pub type KPRIORITY = IO_PRIORITY::KPRIORITY_BOOST;

/// Memory Descriptor List (MDL)
#[repr(C)]
pub struct MDL
{
	Next: *mut MDL,
	Size: i16,
	MdlFlags: i16,
	Process: PEPROCESS,
	MappedSystemVa: PVOID,
	StartVa: PVOID,
	ByteCount: u32,
	ByteOffset: u32,
}

pub type PMDL = *mut MDL;

#[repr(i16)]
pub enum MDL_FLAGS {
  MDL_MAPPED_TO_SYSTEM_VA = 0x0001,
  MDL_PAGES_LOCKED = 0x0002,
  MDL_SOURCE_IS_NONPAGED_POOL = 0x0004,
  MDL_ALLOCATED_FIXED_SIZE = 0x0008,
  MDL_PARTIAL = 0x0010,
  MDL_PARTIAL_HAS_BEEN_MAPPED = 0x0020,
  MDL_IO_PAGE_READ = 0x0040,
  MDL_WRITE_OPERATION = 0x0080,
  MDL_PARENT_MAPPED_SYSTEM_VA = 0x0100,
  MDL_LOCK_HELD = 0x0200,
  MDL_SCATTER_GATHER_VA = 0x0400,
  MDL_IO_SPACE = 0x0800,
  MDL_NETWORK_HEADER = 0x1000,
  MDL_MAPPING_CAN_FAIL = 0x2000,
  MDL_ALLOCATED_MUST_SUCCEED = 0x4000,
}
