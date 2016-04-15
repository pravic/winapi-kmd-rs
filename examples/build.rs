use std::env;

fn main() {
	let target = env::var("TARGET").unwrap();
	let triple: Vec<_> = target.split('-').collect();
	let mapped = match triple.get(0) {
		Some(&"i686") => "i386",
		Some(&"x86_64") => "amd64",
		_ => panic!("unknown architecture of {:?}", target),
	};
	println!("cargo:rustc-link-search=../../../native/win7/{}", mapped);
}
