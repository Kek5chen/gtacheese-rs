#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ansi_term::Color::Red;
use log::LevelFilter;
use std::time::Duration;
use windows::Win32::System::Console::*;

mod cheese;
mod util;

unsafe fn init_debug_console() -> u32 {
    if let Err(e) = env_logger::Builder::new()
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .filter_level(LevelFilter::Debug)
        .try_init()
    {
        println!("Failed to initialize env_logger: {e:?}");
        return 1;
    }

    log::debug!("Initialized environment logger");
    0
}

fn main() {
    unsafe {
        if cfg!(debug_assertions) {
            init_debug_console();
        }

        match cheese::main() {
            Ok(()) => log::info!("Successfully shutting down"),
            Err(e) => {
                if cfg!(debug_assertions) {
                    log::error!("{}{e}", Red.paint("Error caused cheese to crash out of execution. Error was caught but fatal: "));
                    std::thread::sleep(Duration::from_secs(10));
                }
            }
        }
    }
}
