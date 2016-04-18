## Windows Kernel-Mode Drivers written in Rust

[![Join the chat at https://gitter.im/pravic/winapi-kmd-rs](https://badges.gitter.im/pravic/winapi-kmd-rs.svg)](https://gitter.im/pravic/winapi-kmd-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

This library is just a proof-of-concept of the windows kernel-mode drivers, which can be written in Rust programming language.

It contains the types, constants and bindings for the [Windows Driver Kit](https://en.wikipedia.org/wiki/Windows_Driver_Kit)
with target OS starting from Windows XP (x86/x64).


### Getting started

To compile you need the following:

* Nightly Rust with MSVC ABI starting from 2016-04-12 (?), which supports "[is-like-msvc](https://github.com/rust-lang/rust/pull/32823)" target flavor.
* MSVC itself, either VS 2015 or just MSVC Build Tools.
* Rust environment for the Windows drivers: [kmd-env-rs](https://github.com/pravic/kmd-env-rs).

As workaround you can compile drivers as `#[crate_type="staticlib"]` and link them manually (see *examples/03.urandom/build.cmd*).


Setting up:

```
git clone https://github.com/pravic/kmd-env-rs .
git submodule init
git submodule update --recursive
```

[Set](https://github.com/rust-lang-nursery/multirust-rs#directory-overrides) the nightly-msvc Rust toolchain for this repository:

`rustup override add nightly-i686-msvc`

Try to compile example:

```
cd km\examples\01.minimal\
cargo build --release
```

By default, it compiles to x86 mode.
If you need x64, either change `kmd-env-rs/.cargo/config` as following:

```
[build]
target = "x86_64-sys-windows-msvc"
...
```

or use [RUST_TARGET_PATH](https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md):

```
set RUST_TARGET_PATH=C:/path/to/kmd-env-rs/.cargo`
cargo build --release --target i686-sys-windows-msvc
cargo build --release --target x86_64-sys-windows-msvc
```



### Examples

See [examples](https://github.com/pravic/winapi-kmd-rs/tree/master/examples) folder with a driver samples and screenshots.


### [Reference documentation](http://pravic.github.io/winapi-kmd-rs/).


#### Acknowledges

In memory of [Four-F](https://web.archive.org/web/20130530073702/http://www.freewebs.com/four-f/) - the author of tutorials about kernel mode drivers
development in assembly language (2002-2005).
