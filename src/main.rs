#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"haha lmao";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i,&byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xc;j
        }
    }
 
    loop {}
}

//PanicInfo: 
//Contains the file and lne where the panic happened and the optional panic message
//The function should never return, so its marked as a diverging function by returning "never" type ! 
#[panic_handler]  
fn panic(_info: &PanicInfo) -> ! { 
    loop {}
}