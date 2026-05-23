use super::*;

// OPFF
pub unsafe extern "C" fn bomb_frame(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let ENTRY_ID = get_entry_id(owner_boma);
        let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
        let status_kind = StatusModule::status_kind(owner_boma);

        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("burst"), 0.0, 1.0, false, 0.0, false, false);
        }

        if motion_kind == hash40("fly") { 
            let boma_match = distance_formula_weapon(weapon.module_accessor, owner_boma, 24.0);
            if boma_match.is_some() {
                MotionModule::change_motion(weapon.module_accessor, Hash40::new("burst"), 0.0, 1.0, false, 0.0, false, false);
            }
        }

        // RESET BOMB FLAG ON HIT
        if WorkModule::is_flag(weapon.module_accessor, *WEAPON_GAMEWATCH_BOMB_STATUS_WORK_FLAG_DAMAGE) {
            BOMB_OUT[ENTRY_ID] = false;
        }
    }
}

// ON START
pub unsafe extern "C" fn bomb_start(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        
    }
}

pub fn install() {
    Agent::new("gamewatch_bomb")
        .on_line(Main, bomb_frame)
        .on_start(bomb_start)
        .install();
}