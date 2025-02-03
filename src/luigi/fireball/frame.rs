use super::*;

// OPWF
pub unsafe extern "C" fn fireball_frame(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    }
}

// START
pub unsafe extern "C" fn fireball_start(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let ENTRY_ID = get_entry_id(owner_boma);
        FIREBALL_SPEED_X[ENTRY_ID] = 0.0;
    }
}

pub fn install() {
    Agent::new("luigi_fireball")
        .on_line(Main, fireball_frame)
        .on_start(fireball_start)
        .install();
}