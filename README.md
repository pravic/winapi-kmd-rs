## Windows Kernel-Mode Drivers written in Rust

This library is just a proof-of-concept of windows kernel-mode drivers, which can be written in Rust programming language.

It contains the types, constants and bindings for the [Windows Driver Kit](https://en.wikipedia.org/wiki/Windows_Driver_Kit) with target OS starting from Windows XP (x86/x64).

To compile you need the following:

* Nightly Rust with MSVC ABI starting from 2016-04-12 (?), which supports "[is-like-msvc](https://github.com/rust-lang/rust/pull/32823)" target flavor.
* MSVC itself, either VS 2015 or just MSVC Build Tools.
* Rust environment for the Windows drivers: [kmd-env-rs](https://github.com/pravic/kmd-env-rs)

TBD.

### Examples

[Here](https://github.com/pravic/winapi-km-rs/tree/master/examples) is a three basic driver samples.

## [Reference](http://pravic.github.io/winapi-km-rs/)

