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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}