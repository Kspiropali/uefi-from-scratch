#![no_main]
#![no_std]
#![warn(dead_code)]

use uefi_from_scratch::{EFI_HANDLE, EFI_STATUS, EFI_SYSTEM_TABLE};
use uefi_from_scratch::r#enum::{EFI_BACKGROUND_BLACK, EFI_SUCCESS, EFI_YELLOW};

#[panic_handler]
#[no_mangle]
#[cfg(not(test))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[allow(unreachable_code)]
#[no_mangle]
unsafe fn efi_main(_handle: EFI_HANDLE, st: *mut EFI_SYSTEM_TABLE) -> EFI_STATUS {
    // Get the system table from the pointer
    let system_table = &mut *st as *mut EFI_SYSTEM_TABLE;
    // Get the console output protocol
    let con_out = (*system_table).ConOut;

    //reset UEFI buffer output
    (*con_out).reset(true).expect("Could not reset console");
    // Set the color to blue
    (*con_out).set_color(EFI_YELLOW, EFI_BACKGROUND_BLACK).expect("Could not set color");
    // Write "Hello, world!" to the console
    (*con_out).write("Hello, world!\r\n").expect("Could not write to console");

    loop {}
    EFI_SUCCESS
}