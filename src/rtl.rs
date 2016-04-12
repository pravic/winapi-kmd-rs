//! NT runtime routines.

extern "system"
{
	/// Returns a random number that was generated from a given `seed` value in the range `[0..MAXLONG-1]`.
	pub fn RtlRandom(Seed: *mut u32)	 -> u32;
	/// Returns a random number that was generated from a given `seed` value in the range `[0..MAXLONG-1]`.
	pub fn RtlRandomEx(Seed: *mut u32) -> u32;
	/// A simple uniform random number generator, based on D.H. Lehmer's 1948 alrogithm.
	pub fn RtlUniform(Seed: *mut u32)	 -> u32;
}
