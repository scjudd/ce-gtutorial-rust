mod winapi;

fn find_process(name: &str) -> Option<winapi::DWORD> {
    use winapi::*;

    unsafe {
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        let mut proc: PROCESSENTRY32 = std::mem::zeroed();
        proc.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        let mut ok = Process32First(handle, &mut proc);

        loop {
            if ok == 0 {
                let err = GetLastError();

                if err == ERROR_NO_MORE_FILES {
                    return None;
                }

                // Anything other than ERROR_NO_MORE_FILES is a programming error.
                panic!("find_process Process32(First|Next) error: {}", err);
            }

            let exe = std::ffi::CStr::from_ptr(&proc.szExeFile as *const CHAR);
            if exe.to_str().expect("Invalid process name") == name {
                return Some(proc.th32ProcessID);
            }

            ok = Process32Next(handle, &mut proc);
        }
    }
}

fn patch_memory(pid: winapi::DWORD) {
    use winapi::*;

    unsafe {
        let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
        if handle == std::ptr::null_mut() {
            let err = GetLastError();
            panic!("patch_memory OpenProcess error: {}", err);
        }

        let addr = 0x10003D1D9 as LPVOID;
        // Original code: [0x83, 0x43, 0x6c, 0x01]
        let buf: [u8; 4] = [0x90, 0x90, 0x90, 0x90];
        let buf_ptr = &buf as *const _ as *const c_void;

        let size: SIZE_T = 4;
        let mut written: SIZE_T = 0;
        let written_ptr: *mut SIZE_T = &mut written;

        let ok = WriteProcessMemory(handle, addr, buf_ptr, size, written_ptr);
        if ok == 0 {
            let err = GetLastError();
            panic!("patch_memory WriteProcessMemory error: {}", err);
        }
        
        let ok = CloseHandle(handle);
        if ok == 0 {
            let err = GetLastError();
            panic!("patch_memory CloseHandle error: {}", err);
        }
    }
}

fn main() {
    let pid = find_process("gtutorial-x86_64.exe").expect("no matching process found");
    println!("Found process with PID: {}", pid);
    patch_memory(pid);
    println!("Patching game...");
}
