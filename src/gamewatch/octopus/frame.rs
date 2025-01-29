use super::*;

// OPFF
pub unsafe extern "C" fn octopus_frame(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let ENTRY_ID = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if OCTOPUS[ENTRY_ID] {
            StatusModule::change_status_request_from_script(weapon.module_accessor, WEAPON_GAMEWATCH_OCTOPUS_STATUS_KIND_ATTACKAIRN, true);
            OCTOPUS[ENTRY_ID] = false;
        }
    }
}

pub fn install() {
    Agent::new("gamewatch_octopus")
        .on_line(Main, octopus_frame)
        .install();
}