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

// UINTN can be used for both 64-Bit ( 8 Bytes ) and 32-Bit ( 4 Bytes ).
// We set this for 64-Bit since this tutorial series is 64-Bit only.
// UEFI 2.9 Specs PDF Page 20
pub type UINTN = UINT64;

#[allow(clippy::upper_case_acronyms)]
type BOOLEAN = c_uchar;

pub type EFI_HANDLE = *mut core::ffi::c_void;
pub type EFI_STATUS = UINT64;

#[repr(C)]
// This struct is a placeholder and not usable at this time
// The code will not compile without it though.
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {}

type EFI_TEXT_RESET = extern "efiapi" fn(*mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, BOOLEAN) -> EFI_STATUS;
type EFI_TEXT_STRING = extern "efiapi" fn(*mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *mut CHAR16) -> EFI_STATUS;
type EFI_TEXT_TEST_STRING = extern "efiapi" fn(*mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *mut CHAR16) -> EFI_STATUS;
type EFI_TEXT_QUERY_STRING = extern "efiapi" fn(*mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, UINTN, *mut UINTN, *mut UINTN) -> EFI_STATUS;
type EFI_TEXT_SET_MODE = extern "efiapi" fn(*mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, UINTN) -> EFI_STATUS;
type EFI_TEXT_SET_ATTRIBUTE = extern "efiapi" fn(*mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, UINTN) -> EFI_STATUS;

#[repr(C)]
// We are forward declaring this struct so that the two function typedefs can operate.
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset: EFI_TEXT_RESET,
    pub OutputString: EFI_TEXT_STRING,
    pub TestString: EFI_TEXT_TEST_STRING,
    pub QueryMode: EFI_TEXT_QUERY_STRING,
    pub SetMode: EFI_TEXT_SET_MODE,
    pub SetAttribute: EFI_TEXT_SET_ATTRIBUTE,
}

// The struct for the EFI Text Output protocols.
// UEFI 2.9 Specs PDF Page 449
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

    pub fn set_color(&mut self, foreground: u8, background: u8) -> Result<(), EFI_STATUS> {
        let status = (self.SetAttribute)(self, ((background << 4) | foreground) as UINTN);
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }
}

#[repr(C)]
// This is the main EFI header for all of the EFI.
// UEFI 2.9 Specs PDF Page 93
pub struct EFI_TABLE_HEADER {
    Signature: UINT64,
    Revision: UINT32,
    HeaderSize: UINT32,
    CRC32: UINT32,
    Reserved: UINT32,
}

#[repr(C)]
// EFI has a system and runtime. This system table is the first struct
// called from the main section. Think of it as the entry point
// to all of the EFI functions.
// UEFI 2.9 Specs PDF Page 94
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
// EFI has a system and runtime. This system table is the first struct
// called from the main section. Think of it as the entry point
// to all of the EFI functions.
// UEFI 2.9 Specs PDF Page 94
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