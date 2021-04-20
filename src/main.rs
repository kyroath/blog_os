#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// This is the entry point of the kernel
#[no_mangle] // The name should not be mangled
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop {}
}
