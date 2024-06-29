use crate::cheese::classes::ped_factory::CPedFactory;

pub fn kill() {
    unsafe {
        if let Some(player) = CPedFactory::get_local_player() {
            let _ = player.kill();
        }
    }
}