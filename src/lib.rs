#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![no_main]
#![no_std]
#![allow(non_snake_case)]

pub mod r#enum;

use core::ffi::c_uchar;
use core::ptr;

type CHAR16 = u16;
type UINT32 = u32;
type UINT64 = u64;
#[allow(clippy::upper_case_acronyms)]
type BOOLEAN = c_uchar;

pub type EFI_HANDLE = *mut core::ffi::c_void;
pub type EFI_STATUS = UINT64;

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {}

type EFI_TEXT_RESET = extern "efiapi" fn(*mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, BOOLEAN) -> EFI_STATUS;
type EFI_TEXT_STRING = extern "efiapi" fn(*mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *mut CHAR16) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset: EFI_TEXT_RESET,
    pub OutputString: EFI_TEXT_STRING,
}

impl EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub fn reset(&mut self, extended_verification: bool) -> Result<(), EFI_STATUS> {
        let status = (self.Reset)(self, extended_verification as BOOLEAN);
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn write(&mut self, s: &str) -> Result<(), EFI_STATUS> {
        let mut buf: [CHAR16; 1024] = [0; 1024];
        for (i, c) in s.encode_utf16().enumerate() {
            buf[i] = c;
        }
        let status = (self.OutputString)(self, buf.as_mut_ptr());
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }
}

#[repr(C)]
pub struct EFI_TABLE_HEADER {
    Signature: UINT64,
    Revision: UINT32,
    HeaderSize: UINT32,
    CRC32: UINT32,
    Reserved: UINT32,
}

#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    hrd: EFI_TABLE_HEADER,
    FirmwareVendor: *mut CHAR16,
    FirmwareVersion: UINT32,
    ConsoleInHandle: EFI_HANDLE,
    ConIn: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle: EFI_HANDLE,
    pub ConOut: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
}

#[allow(clippy::new_without_default)]
impl EFI_SYSTEM_TABLE {
    pub fn new() -> EFI_SYSTEM_TABLE {
        EFI_SYSTEM_TABLE {
            hrd: EFI_TABLE_HEADER {
                Signature: 0,
                Revision: 0,
                HeaderSize: 0,
                CRC32: 0,
                Reserved: 0,
            },
            FirmwareVendor: ptr::null_mut(),
            FirmwareVersion: 0,
            ConsoleInHandle: ptr::null_mut(),
            ConIn: ptr::null_mut(),
            ConsoleOutHandle: ptr::null_mut(),
            ConOut: ptr::null_mut(),
        }
    }

    pub fn stdout(&mut self) -> &mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
        unsafe { &mut *self.ConOut }
    }
}