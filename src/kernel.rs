use crate::{
    println,
    serial_println
};
pub fn kmain(){
    serial_println!("[os] init");
    println!("Hello World{}", "!");
    x86_64::instructions::interrupts::int3();
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
}