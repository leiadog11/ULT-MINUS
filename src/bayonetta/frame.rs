use super::*;

// OPFF
pub unsafe extern "C" fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let currentsize = PostureModule::scale(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let ENTRY_ID = get_entry_id(boma);
        let mut stock_count = get_stock_count(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true);
            stock_count = get_stock_count(boma);
        }

        // UP TILT
        if MotionModule::motion_kind(boma) == hash40("attack_hi3") {
            if frame >= 8.0 {
                //StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                let pos_y = PostureModule::pos_y(boma);
                let pos_x = PostureModule::pos_x(boma);
                let pos_z = PostureModule::pos_z(boma);

                let mut newPos = Vector3f{x: pos_x, y: pos_y + 0.2, z: pos_z};
                PostureModule::set_pos(boma, &newPos);
            }
        }

        // 
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
pub unsafe extern "C" fn bayonetta_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        STALL_TIMER[ENTRY_ID] = 0;
        RECEIVED_FINAL_SMASH[ENTRY_ID] = false;
    }
}

pub fn install() {
    Agent::new("bayonetta")
        .on_line(Main, bayonetta_frame)
        .on_start(bayonetta_start)
        .install();
}