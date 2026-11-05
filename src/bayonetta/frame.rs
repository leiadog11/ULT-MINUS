use super::*;

// OPFF
pub unsafe extern "C" fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let mut stock_count = get_stock_count(boma);
        let current_size = PostureModule::scale(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            stock_count = get_stock_count(boma);
        }

        // ON GAME OVER - RESET VARIABLES
        if status_kind == *FIGHTER_STATUS_KIND_DEMO { 
            RECEIVED_FINAL_SMASH[ENTRY_ID] = false;
        }

        // ON GROUND
        if situation_kind == *SITUATION_KIND_GROUND { 
            WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        }

        // DOWN AIR
        if MotionModule::motion_kind(boma) == hash40("attack_air_lw") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::set_rate(boma, 0.2);            
            }
            else {
                MotionModule::set_rate(boma, 2.0);
            }
        }

        // COMEBACK
        if DamageModule::damage(boma, 0) >= 25.0 && stock_count == 1 && !RECEIVED_FINAL_SMASH[ENTRY_ID] {
            get_final_smash(boma);
            RECEIVED_FINAL_SMASH[ENTRY_ID] = true;
        }
        
        // BAYO GROWS LARGER IF NAIR IS HELD
        if MotionModule::motion_kind(boma) == hash40("attack_air_n_hold") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                PostureModule::set_scale(boma, current_size * 1.04, false);
            }
        }
        else {
            PostureModule::set_scale(boma, 1.0, false);
        }
    }
}

// ON START
pub unsafe extern "C" fn bayonetta_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        RECEIVED_FINAL_SMASH[ENTRY_ID] = false;
    }
}

pub fn install() {
    Agent::new("bayonetta")
        .on_line(Main, bayonetta_frame)
        .on_start(bayonetta_start)
    .install();
}