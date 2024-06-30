use crate::cheese::classes::ped_factory::CPedFactory;
use crate::cheese::classes::wanted::CWanted;

pub fn kill() -> anyhow::Result<()> {
    unsafe {
        if let Some(player) = CPedFactory::get_local_player() {
            player.kill()?;
        }
    }
    Ok(())
}

pub fn reload_all() -> anyhow::Result<()> {
    unsafe { if let Some(player) = CPedFactory::get_local_player() {} }
    Ok(())
}

pub fn decrease_wanted() -> anyhow::Result<()> {
    unsafe {
        if let Some(wanted) = CWanted::get_local_wanted() {
            if let Some(wanted_level) = wanted.get_wanted_level() {
                wanted.set_wanted_level((wanted_level as i32 - 1).max(0) as u32)?;
            }
        }
    }
    Ok(())
}

pub fn increase_wanted() -> anyhow::Result<()> {
    unsafe {
        if let Some(wanted) = CWanted::get_local_wanted() {
            if let Some(wanted_level) = wanted.get_wanted_level() {
                wanted.set_wanted_level((wanted_level + 1).min(5))?;
            }
        }
    }
    Ok(())
}
