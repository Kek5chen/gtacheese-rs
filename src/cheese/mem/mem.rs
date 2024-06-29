use std::ffi::{c_void, CStr, CString};
use std::mem::{size_of, MaybeUninit, size_of_val};
use windows::Win32::Foundation::{CloseHandle, E_FAIL, HANDLE, HMODULE, MAX_PATH};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use windows::Win32::System::Memory::IsBadReadPtr;
use windows::Win32::System::ProcessStatus::{EnumProcessModules, GetModuleFileNameExA, GetModuleInformation, GetProcessImageFileNameA, MODULEINFO};
use windows::Win32::System::Threading::*;

pub unsafe fn is_addr_valid(addr: usize) -> bool {
    if addr == 0 {
        return false;
    }
    // TODO: actually make this check regions or something
    return true;
}

pub struct Process {
    pub(super) name: String,
    pub(super) pid: u32,
    pub(super) handle: HANDLE,
    pub(super) base_address: usize,
    pub(super) entry_point: usize,
    pub(super) size_of_image: usize,
}

impl Process {
    pub(crate) const fn placeholder() -> Self {
        Process {
            name: String::new(),
            pid: 0,
            handle: HANDLE(0),
            base_address: 0,
            entry_point: 0,
            size_of_image: 0,
        } 
    }
    
    unsafe fn get_process_from_name(name: &str) -> anyhow::Result<PROCESSENTRY32> {
        let name = CString::new(name)?;

        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut entry = PROCESSENTRY32 {
            dwSize: size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        if let Err(e) = Process32First(snapshot, &mut entry) {
            CloseHandle(snapshot)?;
            return Err(e.into());
        }

        loop {
            let exe_name = CStr::from_ptr(entry.szExeFile.as_mut_ptr());

            if name.as_c_str() == exe_name {
                CloseHandle(snapshot)?;
                return Ok(entry)
            }

            if let Err(e) = Process32Next(snapshot, &mut entry) {
                CloseHandle(snapshot)?;
                return Err(e.into());
            }
        }
    }
    
    unsafe fn load_proc_info(hProc: HANDLE) -> anyhow::Result<MODULEINFO> {
        let mut filename: [u8; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let len: usize = GetProcessImageFileNameA(hProc, &mut filename) as usize;
        let filename = CStr::from_bytes_with_nul(&filename[0..=len]);
        
        let mut modules = Vec::<MaybeUninit<HMODULE>>::with_capacity(1000);
        modules.resize(1000, MaybeUninit::uninit());
        
        let mut needed = 0;
        
        EnumProcessModules(hProc, modules.as_mut_ptr() as *mut HMODULE, (modules.len() * size_of::<HMODULE>()) as u32, &mut needed)?;
        
        let modules_loaded = needed as usize / size_of::<HMODULE>();

        if modules_loaded > modules.capacity() {
            anyhow::bail!("Capacity too low. {} modules not enough. Were {}. Too lazy to fix.", modules.capacity(), modules_loaded);
        }
        
        for module in &modules[0..modules_loaded] {
            let mut modname: [u8; MAX_PATH as usize] = [0; MAX_PATH as usize];
            let len = GetModuleFileNameExA(hProc, module.assume_init(), &mut modname) as usize;
            let modname_cstr = CStr::from_bytes_with_nul(&modname[0..=len])?;
            let modname_str = modname_cstr.to_string_lossy();
            
            if !modname_str.ends_with(".exe") {
                continue;
            }
            
            let mut mod_info = MODULEINFO::default();
            GetModuleInformation(hProc, module.assume_init(), &mut mod_info, size_of::<MODULEINFO>() as u32)?;
            
            return Ok(mod_info)
        }
        anyhow::bail!("Did not find base address")
    }

    pub unsafe fn open(name: &str) -> anyhow::Result<Self> {
        let proc = Self::get_process_from_name(name)?;
        let pid = proc.th32ProcessID;

        let handle = OpenProcess(PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_QUERY_INFORMATION, true, pid)?;
        log::debug!("Process \"{}\" found with PID {} and opened", name, pid);

        let proc_info = Self::load_proc_info(handle)?;
        Ok(Self {
            name: name.to_owned(),
            pid,
            handle,
            base_address: proc_info.lpBaseOfDll as usize,
            entry_point: proc_info.EntryPoint as usize,
            size_of_image: proc_info.SizeOfImage as usize,
        })
    }

    pub unsafe fn read<T>(&self, ptr: usize) -> Option<T> {
        let mut buff = MaybeUninit::<T>::uninit();
        let mut number_bytes_read = 0;

        if let Err(e) = ReadProcessMemory(
            self.handle,
            ptr as *const c_void,
            buff.as_mut_ptr() as *mut c_void,
            size_of::<T>(),
            Some(&mut number_bytes_read),
        ) {
            log::error!("Tried to read memory, but failed: {e}");
            return None;
        }

        if number_bytes_read == size_of::<T>() {
            Some(buff.assume_init())
        } else {
            None
        }
    }

    pub unsafe fn read_raw(&self, ptr: usize, size: usize) -> Option<Vec<u8>> {
        let mut buff = vec![0; size];
        let mut number_bytes_read = 0;

        if let Err(e) = ReadProcessMemory(
            self.handle,
            ptr as *const c_void,
            buff.as_mut_ptr() as *mut c_void,
            size,
            Some(&mut number_bytes_read),
        ) {
            log::error!("Tried to read memory, but failed: {e}");
            return None;
        }

        if number_bytes_read == size {
            Some(buff)
        } else {
            None
        }
    }

    pub unsafe fn write<T>(&self, ptr: usize, value: T) -> windows::core::Result<()> {
        let mut number_bytes_written = 0;
        WriteProcessMemory(
            self.handle,
            ptr as *const c_void,
            &value as *const T as *const c_void,
            size_of::<T>(),
            Some(&mut number_bytes_written),
        )?;

        if number_bytes_written != size_of::<T>() {
            Err(windows::core::Error::new(
                E_FAIL,
                format!(
                    "Did not write the full value to memory. ({} instead of {})",
                    number_bytes_written,
                    size_of::<T>()
                ),
            ))
        } else {
            Ok(())
        }
    }
}
