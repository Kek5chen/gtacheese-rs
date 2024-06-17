use log::LevelFilter;
use std::ffi::c_void;
use windows::Win32::Foundation::*;
use windows::Win32::System::Console::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::System::Threading::*;

mod cheese;

unsafe extern "system" fn on_attach(dll: *mut c_void) -> u32 {
    let dll = HINSTANCE(dll as isize);
    if AllocConsole().is_err() {
        println!("Failed to initialize console! Wait, who am I talking to again?");
        return 1;
    }

    if let Err(e) = env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .try_init()
    {
        println!("Failed to initialize env_logger: {e:?}");
        return 1;
    }

    log::debug!("Initialized environment logger");

    cheese::main();

    FreeConsole().unwrap();

    FreeLibraryAndExitThread(dll, 0);
}

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(dll: HINSTANCE, reason: u32, _: *mut c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        DisableThreadLibraryCalls(dll).expect("bruh");
        CreateThread(
            None,
            0,
            Some(on_attach),
            Some(dll.0 as *mut c_void),
            THREAD_CREATION_FLAGS(0),
            None,
        ).unwrap();
    }

    TRUE
}
