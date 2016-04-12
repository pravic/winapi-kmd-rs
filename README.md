## Windows Kernel-Mode Drivers written in Rust

This library is just a proof-of-concept of the windows kernel-mode drivers, which can be written in Rust programming language.

It contains the types, constants and bindings for the [Windows Driver Kit](https://en.wikipedia.org/wiki/Windows_Driver_Kit)
with target OS starting from Windows XP (x86/x64).


### Getting started

To compile you need the following:

* Nightly Rust with MSVC ABI starting from 2016-04-12 (?), which supports "[is-like-msvc](https://github.com/rust-lang/rust/pull/32823)" target flavor.
* MSVC itself, either VS 2015 or just MSVC Build Tools.
* Rust environment for the Windows drivers: [kmd-env-rs](https://github.com/pravic/kmd-env-rs).

Setting up:

```
git clone https://github.com/pravic/kmd-env-rs .
git submodule init
git submodule update --recursive
```

Set the nightly-msvc Rust toolchain [for this](https://github.com/rust-lang-nursery/multirust-rs#directory-overrides) repository:

`rustup override add nightly-i686-msvc`

Try to compile example:

```
cd km\examples\01.minimal\
cargo build --release
```

If linker fails with error *cannot open input file 'ntoskrnl.lib'*,
open `kmd-env-rs/.cargo/config` file and replace `../../../` with the full path to the *kmd-env-rs* directory.


### Examples

See [examples](https://github.com/pravic/winapi-kmd-rs/tree/master/examples) folder with a driver samples and screenshots.


### [Reference documentation](http://pravic.github.io/winapi-kmd-rs/).


#### Acknowledges

In memory of [Four-F](http://four-f.narod.ru/) - the author of tutorials about kernel mode drivers development in assembly language (2002-2005).
