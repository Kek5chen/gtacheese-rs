use crate::cheese::classes::inventory::CInventory;
use crate::cheese::classes::ped::CPed;
use crate::cheese::classes::ped_factory::CPedFactory;
use crate::cheese::classes::wanted::CWanted;

pub fn kill() -> anyhow::Result<()> {
    unsafe {
        if let Some(player) = CPedFactory::get_local_player() {
            player.up().kill()?;
        }
    }
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

pub unsafe fn seatbelt(state: bool) -> anyhow::Result<()> {
    if let Some(local_player) = CPed::local_player() {
        local_player.set_seatbelt(state)?;
    }
    Ok(())
}

pub unsafe fn godmode(state: bool) -> anyhow::Result<()> {
    if let Some(local_player) = CPed::local_player() {
        if state {
            local_player.up().set_health(100000.)?;
        } else if let Some(max_health) = local_player.up().get_max_health() {
            if let Some(health) = local_player.up().get_health() {
                if health > max_health {
                    local_player.up().set_health(max_health)?;
                }
            }
        }
    }
    Ok(())
}

pub unsafe fn never_wanted(state: bool) -> anyhow::Result<()> {
    if state {
        if let Some(wanted) = CWanted::get_local_wanted() {
            wanted.set_wanted_level(0)?;
        }
    }
    Ok(())
}

pub fn refill_ammo() -> anyhow::Result<()> {
    unsafe {
        let ammo_repo = CInventory::get_local_inventory()
            .ok_or_else(|| anyhow::Error::msg("Local Inventory not found"))?
            .get_ammo_repositoy();

        let ammo_amount = ammo_repo.get_ammo_type_amount().unwrap_or_default();
        for i in 0..ammo_amount {
            ammo_repo.set_ammo_amount(i as usize, 9999)?;
        }
    }
    Ok(())
}
