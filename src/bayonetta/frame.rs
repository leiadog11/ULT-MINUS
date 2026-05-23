use super::*;

// OPFF
pub unsafe extern "C" fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(boma);
        let currentsize = PostureModule::scale(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let ENTRY_ID = get_entry_id(boma);
        let mut stock_count = get_stock_count(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            stock_count = get_stock_count(boma);
        }

        // UP TILT
        if MotionModule::motion_kind(boma) == hash40("attack_hi3") {
            if frame >= 8.0 {
                let pos_y = PostureModule::pos_y(boma);
                let pos_x = PostureModule::pos_x(boma);
                let pos_z = PostureModule::pos_z(boma);

                let mut newPos = Vector3f{x: pos_x, y: pos_y + 0.2, z: pos_z};
                PostureModule::set_pos(boma, &newPos);
            }
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
        
        //BAYO GROWS LARGER IF NAIR IS HELD
        if MotionModule::motion_kind(boma) == hash40("attack_air_n_hold") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                PostureModule::set_scale(boma, currentsize*1.04, false);
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