use super::*;

// OPFF
pub unsafe extern "C" fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);

        WorkModule::set_flag(boma, true, 0x200000E9);
        FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(ENTRY_ID as i32), 999.0);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
            GroundModule::set_collidable(boma, true);
        }

        // DEPLETE CURSE_TIMER
        if CURSE_TIMER[ENTRY_ID] > 0 {
            CURSE_TIMER[ENTRY_ID] -= 1; 
        }
    }
}

// ON START
pub unsafe extern "C" fn jack_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        CURSE_TIMER[ENTRY_ID] = 0;
    }
}

pub fn install() {
    Agent::new("jack")
        .on_line(Main, jack_frame)
        .on_start(jack_start)
        .install();
}