#[repr(C)]
pub enum POOL_TYPE
{
	PagedPool,
}

pub type PVOID = *mut u8;

extern "system"
{
	pub fn ExAllocatePoolWithTag(PoolType: POOL_TYPE, NumberOfBytes: usize, Tag: u32) -> PVOID;
	pub fn ExFreePoolWithTag(P: PVOID, Tag: u32);
}
