use super::*;

// OPFF
pub unsafe extern "C" fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);

        // RESET BLOWN UP ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            BLOWN_UP[ENTRY_ID] = false;
        }

        // BLOW UP
        if DamageModule::damage(boma, 0) >= 150.0 {
            if !BLOWN_UP[ENTRY_ID] {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, true);
                BLOWN_UP[ENTRY_ID] = true;
            }
        }

        // NO SPECIAL FALL ON UP B
        if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL || status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL || status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

// ON START
pub unsafe extern "C" fn pichu_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        BLOWN_UP[ENTRY_ID] = false; 
    }
}

pub fn install() {
    Agent::new("pichu")
        .on_line(Main, pichu_frame)
        .on_start(pichu_start)
        .install();
}