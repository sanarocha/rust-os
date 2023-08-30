#![no_std]
//isso aqui tem q adicionar pra linguagem nao compilar nenhuma biblioteca dela q tenha coisa do s.o.
//dai por causa disso começa a dar erro
#![no_main]
// para dizer q usa o start c0

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// ! is the "never" return
static HELLO: &[u8] = b"amo o vivi";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}