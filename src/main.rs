#![no_main]
#![no_std]
#![warn(dead_code)]

use snake::{EFI_HANDLE, EFI_STATUS, EFI_SYSTEM_TABLE};
use snake::r#enum::EFI_SUCCESS;

#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[allow(unreachable_code)]
#[no_mangle]
unsafe fn efi_main(_handle: EFI_HANDLE, st: *mut EFI_SYSTEM_TABLE) -> EFI_STATUS {
    let system_table = &mut *st as *mut EFI_SYSTEM_TABLE;

    //reset UEFI buffer output
    let con_out = (*system_table).ConOut;
    (*con_out).reset(true).expect("Could not reset console");
    (*con_out).write("Hello, world!\r\n").expect("Could not write to console");

    loop {}
    EFI_SUCCESS
}