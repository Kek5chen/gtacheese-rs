use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_END};
use windows::Win32::UI::WindowsAndMessaging::MB_OK;
use crate::cheese::classes::init_classes;
use crate::cheese::classes::ped_factory::CPedFactory;
use crate::cheese::classes::wanted::CWanted;
use crate::cheese::features::versioning::{get_online_version, get_raw_version, get_version, init_versions_or_show_error};
use crate::util::MessageBox;

unsafe fn init_classes_and_check_results() -> bool {
    let init_results = init_classes();
    let mut any_errors = false;

    for (name, result) in init_results {
        match result {
            Ok(()) => log::debug!("Successfully initialized {}", name),
            Err(e) => {
                log::error!("Error initializing {}: {:?}", name, e);
                any_errors = true;
            },
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
        MessageBox("Some signatures weren't found. Please wait for an update.", "Outdated.", MB_OK);
        return Ok(())
    }

    let cur = Instant::now();
    
    MessageBox("I'm waiting here.", "Waiting.", MB_OK);
    
    let fac = CPedFactory::get_instance().unwrap();
    fac.clone_ped(fac.local_player().unwrap(), false, false, false);

    while cur.elapsed().as_secs() < 10 && (GetAsyncKeyState(VK_END.0 as i32) & (1 << 15)) == 0 {
        let wanted = CWanted::get_local_player_wanted();
 
        if let Some(wanted) = wanted {
            if wanted.m_WantedLevel > 0 {
                log::info!("Oh no, you're wanted. Let me take care of that for you! ({}, {})", wanted.m_WantedLevel, wanted.m_nWantedLevel);
                wanted.m_WantedLevel = 0;
                wanted.m_nWantedLevel = 0;
            }
        }
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }
    
    Ok(())
}