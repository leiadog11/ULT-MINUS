use super::*;

// OPFF
pub unsafe extern "C" fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true); // COLLISION FIX
            BLOWN_UP[ENTRY_ID] = false; // RESET BLOWN UP
        }

        // BLOW UP
        if DamageModule::damage(boma, 0) >= 150.0 {
            if !BLOWN_UP[ENTRY_ID] {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, true);
                BLOWN_UP[ENTRY_ID] = true;
                STALL_TIMER[ENTRY_ID] = 0;
            }
        }

        // NO SPECIAL FALL ON UP B
        if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL || status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL || status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
        }

        // DANGER
        if situation_kind == *SITUATION_KIND_AIR {
            if STALL_TIMER[ENTRY_ID] == 720 {
                let dumb = Vector3f{x:0.0,y:10.0,z:0.0};
                EffectModule::req_follow(boma, Hash40::new("sys_flies_up"), Hash40::new("top"), &dumb, &dumb, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                SoundModule::play_se(boma, Hash40::new("se_common_spirits_machstamp_landing"), true, false, false, false, enSEType(0));
                STALL_TIMER[ENTRY_ID] = 721;
            }
            else if STALL_TIMER[ENTRY_ID] == 721 {
                DamageModule::add_damage(boma, 0.5, 0);
                if DamageModule::damage(boma, 0) >= 200.0 {
                    STALL_TIMER[ENTRY_ID] = 0;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, true);
                }
            }
            else {
                STALL_TIMER[ENTRY_ID] += 1;
            }
        }
        else {
            STALL_TIMER[ENTRY_ID] = 0;
            EffectModule::kill_kind(boma, Hash40::new("sys_flies_up"), false, true);
        }
        if status_kind == *FIGHTER_STATUS_KIND_DEMO {
            STALL_TIMER[ENTRY_ID] = 0;
            EffectModule::kill_kind(boma, Hash40::new("sys_flies_up"), false, true);
        }
        if DamageModule::reaction(boma, 0) > 1.0 { 
            STALL_TIMER[ENTRY_ID] = 0;
            EffectModule::kill_kind(boma, Hash40::new("sys_flies_up"), false, true);
        }
    }
}

// ON START
pub unsafe extern "C" fn pichu_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        BLOWN_UP[ENTRY_ID] = false; 
        STALL_TIMER[ENTRY_ID] = 0;
    }
}

pub fn install() {
    Agent::new("pichu")
        .on_line(Main, pichu_frame)
        .on_start(pichu_start)
        .install();
}