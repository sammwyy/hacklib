use std::{mem, os::raw::c_void};

use winapi::{
    shared::{
        minwindef::{FALSE, LPCVOID, LPVOID, TRUE},
        ntdef::NULL,
    },
    um::{
        memoryapi::{ReadProcessMemory, WriteProcessMemory},
        processthreadsapi::OpenProcess,
        winnt::{HANDLE, PROCESS_ALL_ACCESS},
    },
};

pub struct Client {
    pid: u32,
    handle: Option<HANDLE>,
}

impl Client {
    pub fn new(pid: u32) -> Self {
        Client { pid, handle: None }
    }

    pub fn pid(&self) -> u32 {
        return self.pid;
    }

    pub fn init(&mut self) -> bool {
        let pid = self.pid;
        let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid) };

        if handle == NULL {
            return false;
        }

        self.handle = Some(handle);
        return true;
    }

    pub fn read_mem<T: Default>(&self, address: usize) -> T {
        let mut value: T = Default::default();

        let hprocess = self.handle.unwrap();
        let lpbaseaddress = address as *mut c_void;
        let lpbuffer = &mut value as *mut T as LPVOID;
        let nsize = mem::size_of::<T>();
        let lpnumberofbyteswritten = NULL as *mut usize;

        unsafe {
            let result = ReadProcessMemory(
                hprocess,
                lpbaseaddress,
                lpbuffer,
                nsize,
                lpnumberofbyteswritten,
            );

            if result == FALSE {
                let err = std::io::Error::last_os_error();
                println!("{}", err);
            }

            return value;
        }
    }

    pub fn write_mem<T: Default>(&self, address: usize, mut data: T) -> bool {
        let hprocess = self.handle.unwrap();
        let lpbaseaddress = address as *mut c_void;
        let lpbuffer = &mut data as *mut T as LPCVOID;
        let nsize = mem::size_of::<T>();
        let lpnumberofbyteswritten = NULL as *mut usize;

        unsafe {
            let result = WriteProcessMemory(
                hprocess,
                lpbaseaddress,
                lpbuffer,
                nsize,
                lpnumberofbyteswritten,
            );

            if result == FALSE {
                let err = std::io::Error::last_os_error();
                println!("{}", err);
            }

            return result == TRUE;
        }
    }
}
