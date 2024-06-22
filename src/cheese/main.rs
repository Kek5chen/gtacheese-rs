use crate::cheese::classes::init_classes;
use crate::cheese::features::versioning::{
    get_online_version, get_raw_version, get_version, init_versions_or_show_error,
};
use crate::util::MessageBox;
use std::thread::sleep;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_END};
use windows::Win32::UI::WindowsAndMessaging::MB_OK;

unsafe fn init_classes_and_check_results() -> bool {
    let init_results = init_classes();
    let mut any_errors = false;

    for (name, result) in init_results {
        match result {
            Ok(()) => log::debug!("Successfully initialized {}", name),
            Err(e) => {
                log::error!("Error initializing {}: {:?}", name, e);
                any_errors = true;
            }
        }
    }

    any_errors
}

pub unsafe fn main() -> anyhow::Result<()> {
    if !init_versions_or_show_error() {
        return Ok(());
    }

    let version = get_version().unwrap();
    let raw_version = get_raw_version().unwrap();
    let online_version = get_online_version().unwrap();
    log::info!("Version: {version:?} (at: {:?})", version.as_ptr());
    log::info!("Raw Version: {raw_version:?}");
    log::info!("Online Version: {online_version:?}");

    if init_classes_and_check_results() {
        MessageBox(
            "Some signatures weren't found. Please wait for an update.",
            "Outdated.",
            MB_OK,
        );
        return Ok(());
    }

    while GetAsyncKeyState(VK_END.0 as i32) == 0 {
        sleep(Duration::from_millis(100));
    }

    Ok(())
}
