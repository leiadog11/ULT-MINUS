use super::*;

// ON START
pub unsafe extern "C" fn fireball_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let ENTRY_ID = get_entry_id(owner_boma);
        if ICEBALL[ENTRY_ID] {
            IS_ICEBALL = true;
        } else {
            IS_ICEBALL = false;
        }
    }
}

pub fn install() {
    Agent::new("mario_fireball")
        .on_start(fireball_start)
        .install();
}