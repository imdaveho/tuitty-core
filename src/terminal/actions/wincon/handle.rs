//! This module contains shared logic for working with the WINDOWS console.
use winapi::um::{
    minwinbase::SECURITY_ATTRIBUTES,
    fileapi::{ OPEN_EXISTING, CreateFileW },
    handleapi::{ INVALID_HANDLE_VALUE, CloseHandle },
    processenv::GetStdHandle,
    winbase::{ STD_INPUT_HANDLE, STD_OUTPUT_HANDLE },
    winnt::{
        FILE_SHARE_READ, FILE_SHARE_WRITE, 
        GENERIC_READ, GENERIC_WRITE, HANDLE
    },
    wincon::{
        CreateConsoleScreenBuffer, 
        GetConsoleScreenBufferInfo,
        SetConsoleActiveScreenBuffer,
        CONSOLE_TEXTMODE_BUFFER, 
        CONSOLE_SCREEN_BUFFER_INFO,
    },
    consoleapi::{ GetConsoleMode, SetConsoleMode },
};

use winapi::shared::{ minwindef::TRUE, ntdef::NULL };

use std::io::{Error, Result};
use std::ptr::null_mut;
use std::mem::{size_of, zeroed};


pub struct Handle(pub HANDLE);

impl Handle {
    pub fn stdout() -> Result<Handle> {
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);

            if handle == INVALID_HANDLE_VALUE {
                return Err(Error::last_os_error());
            }

        Ok(Handle(handle))
        }
    }

    // https://docs.microsoft.com/en-us/windows/desktop/api/
    // fileapi/nf-fileapi-createfilew
    pub fn conout() -> Result<Handle> {
        let utf16: Vec<u16> = "CONOUT$\0".encode_utf16().collect();
        let utf16_ptr: *const u16 = utf16.as_ptr();

        let handle = unsafe {
            CreateFileW(
                utf16_ptr,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                OPEN_EXISTING,
                0,
                null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::last_os_error());
        }
        Ok(Handle(handle))
    }

    pub fn stdin() -> Result<Handle> {
        unsafe {
            let handle = GetStdHandle(STD_INPUT_HANDLE);

            if handle == INVALID_HANDLE_VALUE {
                return Err(Error::last_os_error());
            }
        Ok(Handle(handle))
        }
    }

    // https://docs.microsoft.com/en-us/windows/desktop/api/
    // fileapi/nf-fileapi-createfilew
    pub fn conin() -> Result<Handle> {
        let utf16: Vec<u16> = "CONIN$\0".encode_utf16().collect();
        let utf16_ptr: *const u16 = utf16.as_ptr();

        let handle = unsafe {
            CreateFileW(
                utf16_ptr,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                OPEN_EXISTING,
                0,
                null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::last_os_error());
        }
        Ok(Handle(handle))
    }

    // https://docs.microsoft.com/en-us/windows/console/
    // createconsolescreenbuffer
    pub fn buffer() -> Result<Handle> {
        let mut security_attr: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: NULL,
            bInheritHandle: TRUE,
        };

        unsafe {
            let handle = CreateConsoleScreenBuffer(
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                &mut security_attr,
                CONSOLE_TEXTMODE_BUFFER,
                NULL,
            );
            if handle == INVALID_HANDLE_VALUE {
                return Err(Error::last_os_error());
            }
            Ok(Handle(handle))
        }
    }

    // https://docs.microsoft.com/en-us/windows/win32/api/
    // handleapi/nf-handleapi-closehandle
    pub fn close(&self) -> Result<()> {
        unsafe {
            if CloseHandle(self.0) == 0 {
                return Err(Error::last_os_error())
            }
        }
        Ok(())
    }

    pub fn show(&self) -> Result<()> {
        unsafe {
            if SetConsoleActiveScreenBuffer(self.0) == 0 {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }

    pub fn get_mode(&self) -> Result<u32> {
        let mut mode = 0;
        unsafe {
            if GetConsoleMode(self.0, &mut mode) == 0 {
                return Err(Error::last_os_error());
            }
        }
        Ok(mode)
    }

    pub fn set_mode(&self, mode: &u32) -> Result<()> {
        unsafe {
            if SetConsoleMode(self.0, *mode) == 0 {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }
}


pub struct ConsoleInfo(CONSOLE_SCREEN_BUFFER_INFO);

impl ConsoleInfo {
    pub fn of(handle: &Handle) -> Result<ConsoleInfo> {
        unsafe {
            let mut info: CONSOLE_SCREEN_BUFFER_INFO = zeroed();
            if GetConsoleScreenBufferInfo(handle.0, &mut info) == 0 {
                return Err(Error::last_os_error());
            }
            Ok(ConsoleInfo(info))
        }
    }

    pub fn buffer_size(&self) -> (i16, i16) {
        let coord = self.0.dwSize;
        (coord.X, coord.Y)
    }

    pub fn terminal_size(&self) -> (i16, i16) {
        let small_rect = self.0.srWindow;
        (
            small_rect.Right - small_rect.Left,
            small_rect.Bottom - small_rect.Top,
        )
    }

    pub fn window_pos(&self) -> (i16, i16, i16, i16) {
        let small_rect = self.0.srWindow;
        (
            small_rect.Left,
            small_rect.Right,
            small_rect.Bottom,
            small_rect.Top
        )
    }

    pub fn attributes(&self) -> u16 {
        self.0.wAttributes
    }

    pub fn cursor_pos(&self) -> (i16, i16) {
        let coord = self.0.dwCursorPosition;
        (coord.X, coord.Y)
    }
}
