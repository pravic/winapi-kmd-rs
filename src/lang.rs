#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn rust_eh_unwind_resume() {}

#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {
	KdPrint!("panic_fmt() -> !");
	loop{}
}
