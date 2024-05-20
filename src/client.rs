use std::{ffi::CStr, mem, os::raw::c_void};

use windows::Win32::{
    Foundation::{HANDLE, WIN32_ERROR},
    System::{
        Diagnostics::{
            Debug::{ReadProcessMemory, WriteProcessMemory},
            ToolHelp::{
                CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32,
                TH32CS_SNAPALL,
            },
        },
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
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
        let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid) };

        if handle.is_err() {
            return false;
        }

        self.handle = Some(handle.unwrap());
        return true;
    }

    pub fn get_loaded_module_names(&self) -> Vec<String> {
        let mut loaded_modules: Vec<String> = Vec::new();

        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPALL, self.pid).unwrap();
            let mut me32 = MODULEENTRY32 {
                dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
                ..Default::default()
            };

            if Module32First(snapshot, &mut me32).is_ok() {
                loop {
                    let name = CStr::from_ptr(me32.szModule.as_ptr()).to_str().unwrap();
                    loaded_modules.push(String::from(name));

                    if !Module32Next(snapshot, &mut me32).is_ok() {
                        break;
                    }
                }
            }
        }

        return loaded_modules;
    }

    pub fn get_module(&self, module_name: &str) -> Result<MODULEENTRY32, WIN32_ERROR> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPALL, self.pid).unwrap();
            let mut me32 = MODULEENTRY32 {
                dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
                ..Default::default()
            };

            if Module32First(snapshot, &mut me32).is_ok() {
                loop {
                    let name = CStr::from_ptr(me32.szModule.as_ptr()).to_str().unwrap();
                    if name.eq_ignore_ascii_case(module_name) {
                        return Ok(me32);
                    }

                    if !Module32Next(snapshot, &mut me32).is_ok() {
                        break;
                    }
                }
            }

            return Err(WIN32_ERROR::from_error(Err("Unknown module").unwrap()).unwrap());
        }
    }

    pub fn get_module_offset(&self, module_name: &str) -> Result<usize, WIN32_ERROR> {
        let me32 = self.get_module(module_name);
        if me32.is_err() {
            return Err(me32.err().unwrap());
        }

        let me32 = me32.unwrap();
        return Ok(me32.modBaseAddr as usize);
    }

    pub fn read<T: Default>(&self, address: usize) -> T {
        let mut value: T = Default::default();

        let hprocess = self.handle.unwrap();
        let lpbaseaddress = address as *mut c_void;
        let lpbuffer = &mut value as *mut T as *mut c_void;
        let nsize = mem::size_of::<T>();

        unsafe {
            let result = ReadProcessMemory(hprocess, lpbaseaddress, lpbuffer, nsize, None);

            if result.is_err() {
                let err = result.unwrap_err();
                println!("{}", err);
            }

            return value;
        }
    }

    pub fn read_str(&self, address: usize, size: usize) -> String {
        let mut str = String::new();
        let mut i = 0;

        while i < size {
            let char_ptr = address + (i * 1);
            let char_index: u8 = self.read(char_ptr);
            let char = char::from(char_index);
            str.push(char);

            i += 1;
        }

        return str;
    }

    pub fn write<T: Default>(&self, address: usize, mut data: T) -> bool {
        let hprocess = self.handle.unwrap();
        let lpbaseaddress = address as *mut c_void;
        let lpbuffer = &mut data as *mut T as *mut c_void;
        let nsize = mem::size_of::<T>();

        unsafe {
            let result = WriteProcessMemory(hprocess, lpbaseaddress, lpbuffer, nsize, None);

            if result.is_err() {
                let err = result.unwrap_err();
                println!("{}", err);
                return false;
            }

            return true;
        }
    }

    pub fn write_str(&self, address: usize, data: String) -> bool {
        let chars = data.chars();
        let mut padding_offset = address;

        for char in chars {
            self.write(padding_offset, char);
            padding_offset += 1;
        }

        return true;
    }
}
