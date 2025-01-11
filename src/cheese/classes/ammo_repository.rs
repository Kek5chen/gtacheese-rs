use crate::cheese::main::PROC;

const OFF_M_AMMO: usize = 0x20;

pub struct CAmmoItemRepository;
pub struct CAmmoItemRepositoryPtr(pub usize);

impl CAmmoItemRepositoryPtr {
    pub unsafe fn get_ammo_type_amount(&self) -> Option<u32> {
        let array = PROC.read::<usize>(self.0)?;
        let mut i: usize = 0;
        while PROC.read::<usize>(array + i * 8usize)? != 0 {
            i += 1;
        }

        Some(i as u32)
    }

    pub unsafe fn set_ammo_amount(&self, idx: usize, amount: u32) -> anyhow::Result<()> {
        let array = PROC
            .read::<usize>(self.0)
            .ok_or(anyhow::Error::msg("Array not found"))?;
        let elem = PROC
            .read::<usize>(array + idx * 8usize)
            .ok_or(anyhow::Error::msg("Array element not found"))
            .and_then(|n| {
                if n == 0 {
                    anyhow::bail!("Array Element was null pointer");
                } else {
                    Ok(n)
                }
            })?;
        PROC.write(elem + OFF_M_AMMO, amount)?;
        Ok(())
    }
}
