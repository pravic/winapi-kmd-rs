//! I/O request packets (IRP).

use ::NTSTATUS;
use ::basedef::*;
use ::event::PKEVENT;
use ::device_object::*;
use ::file_object::*;
use ::basedef::IO_PRIORITY::*;
use ::KIRQL;


pub type PIRP = *mut IRP;
pub type PIO_STACK_LOCATION = *mut IO_STACK_LOCATION;

// NOTE: fastcall is broken: https://github.com/rust-lang/rust/issues/18086
//extern "fastcall" {	fn IofCompleteRequest(Irp: PIRP, PriorityBoost: KPRIORITY_BOOST); }
extern "system"
{
	fn IoCompleteRequest(Irp: PIRP, PriorityBoost: KPRIORITY_BOOST);

	// unfortunately following are macro
	// fn IoGetCurrentIrpStackLocation(Irp: PIRP) -> PIO_STACK_LOCATION;
	// fn IoGetNextIrpStackLocation(Irp: PIRP) -> PIO_STACK_LOCATION;
	// fn IoSetNextIrpStackLocation(Irp: PIRP);
	// fn IoSkipCurrentIrpStackLocation(Irp: PIRP);
}

/// `IRP` Major Function Codes.
#[repr(u8)]
pub enum IRP_MJ
{
	CREATE,
	CREATE_NAMED_PIPE,
	CLOSE,
	READ,
	WRITE,
	QUERY_INFORMATION,
	SET_INFORMATION,
	QUERY_EA,
	SET_EA,
	FLUSH_BUFFERS,
	QUERY_VOLUME_INFORMATION,
	SET_VOLUME_INFORMATION,
	DIRECTORY_CONTROL,
	FILE_SYSTEM_CONTROL,
	DEVICE_CONTROL,
	INTERNAL_DEVICE_CONTROL,
	SHUTDOWN,
	LOCK_CONTROL,
	CLEANUP,
	CREATE_MAILSLOT,
	QUERY_SECURITY,
	SET_SECURITY,
	POWER,
	SYSTEM_CONTROL,
	DEVICE_CHANGE,
	QUERY_QUOTA,
	SET_QUOTA,
	PNP,
	MAXIMUM_FUNCTION,
}

/// The `IRP` structure is a partial opaque structure that represents an I/O request packet.
#[repr(C)]
pub struct IRP
{
	pub Type: u16,
	pub Size: u16,
	/// Pointer to an `MDL` describing a user buffer, if the driver is using direct I/O.
	pub MdlAddress: PVOID,
	/// Flags word - used to remember various flags.
	pub Flags: u32,
	/// Pointer to a system-space buffer if the driver is using buffered I/O.
	pub SystemBuffer: PVOID,
	pub ThreadListEntry: LIST_ENTRY,
	/// I/O status - final status of operation.
	pub IoStatus: IO_STATUS_BLOCK,
	/// Indicates the execution mode of the original requester of the operation.
	pub RequestorMode: KPROCESSOR_MODE,
	/// If set to `TRUE`, a driver has marked the IRP pending.
	pub PendingReturned: bool,
	/// Stack state information.
	pub StackCount: i8,
	/// Stack state information.
	pub CurrentLocation: i8,
	/// If set to `TRUE`, the IRP either is or should be canceled.
	pub Cancel: bool,
	/// Irql at which the cancel spinlock was acquired.
	pub CancelIrql: KIRQL,
	pub ApcEnvironment: u8,
	/// Allocation control flags.
	pub AllocationFlags: u8,
	/// User parameters.
	pub UserIosb: PIO_STATUS_BLOCK,
	pub UserEvent: PKEVENT,

	// union {
	pub UserApcRoutine: PIO_APC_ROUTINE,
	pub UserApcContext: PVOID,
	// } Overlay

	/// Contains the entry point for a driver-supplied `Cancel` routine to be called if the IRP is canceled.
	pub CancelRoutine: PDRIVER_CANCEL,
	/// Contains the address of an output buffer for `IRP_MJ_DEVICE_CONTROL`.
	pub UserBuffer: PVOID,

	/// Kernel structures.
	// union {
	pub Overlay: _IRP_OVERLAY,
	// } Tail
}

/// Kernel structures for IRP.
#[repr(C)]
pub struct _IRP_OVERLAY
{
	pub DriverContext: [PVOID; 4],
	pub Thread: PETHREAD,
	pub AuxiliaryBuffer: PVOID,
	pub ListEntry: LIST_ENTRY,
	/// Current stack location.
	pub CurrentStackLocation: PIO_STACK_LOCATION,
	pub OriginalFileObject: PFILE_OBJECT,
}

/// I/O Stack Locations.
#[repr(C)]
pub struct IO_STACK_LOCATION
{
	/// The IRP major function code indicating the type of I/O operation to be performed.
	pub MajorFunction: u8,
	/// A subfunction code for `MajorFunction`.
	pub MinorFunction: u8,
	/// Request-type-specific values (see [DEVICE_FLAGS](../device_object/enum.DEVICE_FLAGS.html)).
	pub Flags: u8,
	pub Control: u8,

	/// A union that depends on the major and minor IRP function code values
	/// contained in `MajorFunction` and `MinorFunction`.
	// union Parameters
	pub Parameters: [PVOID; 4],

	/// A pointer to the driver-created `DEVICE_OBJECT` structure
	/// representing the target physical, logical, or virtual device for which this driver is to handle the IRP.
	pub DeviceObject: PDEVICE_OBJECT,
	/// A pointer to a `FILE_OBJECT` structure that represents the file object, if any, that is associated with `DeviceObject` pointer.
	pub FileObject: PFILE_OBJECT,
	/// The following routine is invoked depending on the flags in the above `Flags` field.
	pub CompletionRoutine: PIO_COMPLETION_ROUTINE,
	/// The following is used to store the address of the context parameter that should be passed to the `CompletionRoutine`.
	pub Context: PVOID,
}

/// Parameters for `IRP_MJ_READ`.
#[repr(C)]
pub struct _IO_STACK_LOCATION_READ
{
	pub Length: u32,
	pub Key: u32,
	pub ByteOffset: i64,
}


impl IRP {
	/// Returns a pointer to the caller's stack location in the given `IRP`.
	pub fn get_current_stack_location(&mut self) -> &mut IO_STACK_LOCATION {
		unsafe { &mut *self.Overlay.CurrentStackLocation }
	}

	/// Indicates that the caller has completed all processing for a given I/O request
	/// and is returning the given IRP to the I/O manager.
	pub fn complete_request(&mut self, Status: NTSTATUS) -> NTSTATUS {
		self.IoStatus.Status = Status;
		unsafe { IoCompleteRequest(self, IO_NO_INCREMENT) };
		return Status;
	}
}

impl IO_STACK_LOCATION {
	/// Access parameters for `IRP_MJ_READ`.
	pub fn ParametersRead(&mut self) -> &mut _IO_STACK_LOCATION_READ {
		unsafe { ::core::mem::transmute(&mut self.Parameters) }
	}
}
