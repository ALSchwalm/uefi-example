#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_efiapi)]

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "efiapi" fn efi_main() {
    let x = [0u8; 4096];
}
