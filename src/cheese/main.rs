use std::time::Duration;
use crate::cheese::features::versioning::{get_online_version, get_raw_version, get_version};

pub unsafe fn main() -> anyhow::Result<()> {
    let version = get_version()?;
    let raw_version = get_raw_version()?;
    let online_version = get_online_version()?;
    log::info!("Version: {version:?} (at: {:?})", version.as_ptr());
    log::info!("Raw Version: {raw_version:?}");
    log::info!("Online Version: {online_version:?}");
    
    std::thread::sleep(Duration::from_secs(15));
    
    Ok(())
}