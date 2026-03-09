use super::*;

// OPFF
pub unsafe extern "C" fn pit_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            ArticleModule::remove_exist(boma, *FIGHTER_PIT_GENERATE_ARTICLE_CHARIOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            SHIELD_ON[ENTRY_ID] = false;
        }

        // CANCEL NAIR WITH JUMP
        if motion_kind == hash40("attack_air_n") {
            cancel_with_jump(boma, 6.0);
        }

        // ON HIT
        if DamageModule::reaction(boma, 0) > 1.0 {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_b"), true, true);
            macros::STOP_SE(fighter, Hash40::new("se_pit_flight_wings"));
        }

        // GRAB CLIFF
        if situation_kind == *SITUATION_KIND_CLIFF {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_b"), true, true);
            macros::STOP_SE(fighter, Hash40::new("se_pit_flight_wings"));
        }

        // REGENERATE SHIELD
        if !SHIELD_ON[ENTRY_ID] && SHIELD_LIFE[ENTRY_ID] < 600 {
            SHIELD_LIFE[ENTRY_ID] += 1;
        }
    }
}

// ON START
pub unsafe extern "C" fn pit_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        SHIELD_LIFE[ENTRY_ID] = 600;
        BREAK_WAIT_TIME[ENTRY_ID] = 120;
        SHIELD_ON[ENTRY_ID] = false;
    }
}

pub fn install() {
    Agent::new("pit")
        .on_line(Main, pit_frame)
        .on_start(pit_start)
        .install();
}