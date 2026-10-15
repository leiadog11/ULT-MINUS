use super::*;

// OPFF
pub unsafe extern "C" fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);
        let frame = MotionModule::frame(boma);

        WorkModule::set_flag(boma, true, 0x200000E9);
        FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(ENTRY_ID as i32), 999.0);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
            CURSE_TIMER[ENTRY_ID] = 0;
            HAS_FINAL_SMASH[ENTRY_ID] = false;
        }

        // ON GAME OVER - RESET VARIABLES
        if status_kind == *FIGHTER_STATUS_KIND_DEMO { 
            CURSE_TIMER[ENTRY_ID] = 0;
            HAS_FINAL_SMASH[ENTRY_ID] = false;
        }

        // DEPLETE CURSE_TIMER
        if CURSE_TIMER[ENTRY_ID] > 0 {
            CURSE_TIMER[ENTRY_ID] -= 1; 
        }

        if HAS_FINAL_SMASH[ENTRY_ID] {
            if status_kind == *FIGHTER_STATUS_KIND_FINAL {
                HAS_FINAL_SMASH[ENTRY_ID] = false;
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68), 7);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn jack_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        CURSE_TIMER[ENTRY_ID] = 0;
        HAS_FINAL_SMASH[ENTRY_ID] = false;
    }
}

pub fn install() {
    Agent::new("jack")
        .on_line(Main, jack_frame)
        .on_start(jack_start)
        .install();
}