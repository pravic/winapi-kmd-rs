//! Data shared between kernel and user mode.


/// System time is a count of 100-nanosecond intervals since January 1, 1601.
pub type SYSTEMTIME = i64;


/// Dystem time structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KSYSTEM_TIME
{
	LowPart: u32,
	High1Time: i32,
	High2Time: i32,
}

#[repr(C)]
pub enum NT_PRODUCT_TYPE
{
	NtProductWinNt = 1,
	NtProductLanManNt,
	NtProductServer
}

#[repr(C)]
pub enum ALTERNATIVE_ARCHITECTURE_TYPE
{
	StandardDesign,
	NEC98x86,
	EndAlternatives,
}

/// The data shared between kernel and user mode.
#[repr(C)]
pub struct KUSER_SHARED_DATA
{
	pub TickCountLowDeprecated: u32,
	pub TickCountMultiplier: u32,

	/// Current 64-bit interrupt time in 100ns units.
	pub InterruptTime: KSYSTEM_TIME,
	/// Current 64-bit system time in 100ns units.
	pub SystemTime: KSYSTEM_TIME,
	/// Current 64-bit time zone bias.
	pub TimeZoneBias: KSYSTEM_TIME,

	pub ImageNumberLow: u16,
	pub ImageNumberHigh: u16,

	pub NtSystemRoot: [u16; 260],

	pub MaxStackTraceDepth: u32,
	pub CryptoExponent: u32,
	pub TimeZoneId: u32,
	pub LargePageMinimum: u32,
	pub Reserved2: [u32; 7],

	pub NtProductType: NT_PRODUCT_TYPE,
	pub ProductTypeIsValid: bool,
	pub NtMajorVersion: u32,
	pub NtMinorVersion: u32,

	pub ProcessorFeatures: [bool; 64],
	pub Reserved1: u32,
	pub Reserved3: u32,
	pub TimeSlip: u32,

	pub AlternativeArchitecture: ALTERNATIVE_ARCHITECTURE_TYPE,
	pub SystemExpirationDate: u64,
	pub SuiteMask: u32,

	/// True if a kernel debugger is connected/enabled.
	pub KdDebuggerEnabled: bool,
	pub NXSupportPolicy: u8,
	pub ActiveConsoleId: u32,
	pub DismountCount: u32,
	pub ComPlusPackage: u32,
	pub LastSystemRITEventTickCount: u32,
	pub NumberOfPhysicalPages: u32,

	/// True if the system was booted in safe boot mode.
	pub SafeBootMode: bool,
	pub TraceLogging: u32,

	pub TestRetInstruction: u64,
	pub SystemCall: u32,
	pub SystemCallReturn: u32,
	pub SystemCallPad: [u64; 3],

	/// The 64-bit tick count.
	pub TickCount: KSYSTEM_TIME,

	/// Cookie for encoding pointers system wide.
	pub Cookie: u32,

	// NT 6.0+:

			#[cfg(target_arch = "x86_64")]
	pub Wow64SharedInformation: [u32; 16],
			#[cfg(target_arch = "x86_64")]
	pub UserModeGlobalLogger: [u16; 8],
			#[cfg(target_arch = "x86_64")]
	pub HeapTracingPid: [u32; 2],
			#[cfg(target_arch = "x86_64")]
	pub CritSecTracingPid: [u32; 2],
			#[cfg(target_arch = "x86_64")]
	pub ImageFileExecutionOptions: u32,
			#[cfg(target_arch = "x86_64")]
	pub ActiveProcessorAffinity: u64,
			#[cfg(target_arch = "x86_64")]
	pub InterruptTimeBias: u64,
}

#[cfg(target_arch = "x86")]
const KI_USER_SHARED_DATA: *const KUSER_SHARED_DATA = 0xffdf_0000 as *const KUSER_SHARED_DATA;

#[cfg(target_arch = "x86_64")]
const KI_USER_SHARED_DATA: *const KUSER_SHARED_DATA = 0xFFFF_F780_0000_0000 as *const KUSER_SHARED_DATA;

impl KUSER_SHARED_DATA {

	/// Get reference to the mapped shared data.
	pub fn get() -> &'static KUSER_SHARED_DATA {
		unsafe { &*KI_USER_SHARED_DATA }
	}
}

/// Convert `KSYSTEM_TIME` to `i64`
#[cfg(target_arch = "x86_64")]
impl ::core::convert::From<KSYSTEM_TIME> for SYSTEMTIME {
	fn from(time: KSYSTEM_TIME) -> Self {
		unsafe { *(&time as *const KSYSTEM_TIME as *const SYSTEMTIME) }
	}
}

/// Convert `KSYSTEM_TIME` to `i64`
#[cfg(target_arch = "x86")]
impl ::core::convert::From<KSYSTEM_TIME> for SYSTEMTIME {
	fn from(time: KSYSTEM_TIME) -> Self {
		// FIXME: is any `volatile` or memory fenses need here?
		let mut lo;
		let mut hi;
		loop {
			hi = time.High1Time;
			lo = time.LowPart;
			if hi == time.High2Time {
				break;
			}
		}
		return (hi as i64) << 32 | lo as i64
	}
}
