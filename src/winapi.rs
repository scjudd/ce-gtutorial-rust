#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type c_char = i8;
pub type c_int = i32;
pub type c_long = i32;
pub type c_ulong = u32;
pub enum c_void {}

pub type BOOL = c_int;
pub type CHAR = c_char;
pub type DWORD = c_ulong;
pub type LONG = c_long;
pub type ULONG_PTR = usize;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type SIZE_T = ULONG_PTR;
pub type HANDLE = *mut c_void;
pub type LPPROCESSENTRY32 = *mut PROCESSENTRY32;

pub const TH32CS_SNAPPROCESS: DWORD = 0x00000002;

pub const ERROR_NO_MORE_FILES: DWORD = 18;

pub const PROCESS_ALL_ACCESS: DWORD = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0xFFFF;
pub const STANDARD_RIGHTS_REQUIRED: DWORD = 0x000F0000;
pub const SYNCHRONIZE: DWORD = 0x00100000;

#[repr(C)]
pub struct PROCESSENTRY32 {
    pub dwSize: DWORD,
    pub cntUsage: DWORD,
    pub th32ProcessID: DWORD,
    pub th32DefaultHeapID: ULONG_PTR,
    pub th32ModuleID: DWORD,
    pub cntThreads: DWORD,
    pub th32ParentProcessID: DWORD,
    pub pcPriClassBase: LONG,
    pub dwFlags: DWORD,
    pub szExeFile: [CHAR; 260],
}

extern "system" {
    pub fn CreateToolhelp32Snapshot(dwFlags: DWORD, th32ProcessID: DWORD) -> HANDLE;
    pub fn Process32First(hSnapshot: HANDLE, lppe: LPPROCESSENTRY32) -> BOOL;
    pub fn Process32Next(hSnapshot: HANDLE, lppe: LPPROCESSENTRY32) -> BOOL;
    pub fn GetLastError() -> DWORD;
    pub fn OpenProcess(dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE;
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
    pub fn WriteProcessMemory(
        hProcess: HANDLE,
        lpBaseAddress: LPVOID,
        lpBuffer: LPCVOID,
        nSize: SIZE_T,
        lpNumberOfBytesWritten: *mut SIZE_T,
    ) -> BOOL;
}
