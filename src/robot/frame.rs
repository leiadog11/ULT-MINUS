use super::*;

// OPFF
pub unsafe extern "C" fn robot_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true);
        }

        // GYRO
        if ItemModule::is_have_item(boma, 0)  { 
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) || 
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                GYRO_LIFE[ENTRY_ID] = 360;
            }
        }
        else {
            if GYRO_LIFE[ENTRY_ID] > 0 {
                GYRO_LIFE[ENTRY_ID] -= 1;
                let opponent_bomas = get_opponent_bomas(boma);
                for opponent_boma in opponent_bomas.iter() { 
                    if DamageModule::reaction(*opponent_boma, 0) > 1.0 { 
                        if motion_kind != hash40("attack_air_b") && motion_kind != hash40("attack_air_f") && motion_kind != hash40("attack_air_n") &&  motion_kind != hash40("attack_air_lw") && 
                        motion_kind != hash40("attack_air_hi") && motion_kind != hash40("attack_11") && motion_kind != hash40("attack_12") && motion_kind != hash40("attack_13") &&
                        motion_kind != hash40("attack_dash") && motion_kind != hash40("catch_attack") && motion_kind != hash40("cliff_attack") && motion_kind != hash40("slip_attack") &&
                        motion_kind != hash40("throw_b") && motion_kind != hash40("throw_hi") && motion_kind != hash40("throw_f") && motion_kind != hash40("throw_lw") &&
                        motion_kind != hash40("attack_s3_s") && motion_kind != hash40("attack_s3_hi") && motion_kind != hash40("attack_s3_lw") && motion_kind != hash40("attack_lw3") && 
                        motion_kind != hash40("attack_hi3") && motion_kind != hash40("attack_s4_s") && motion_kind != hash40("attack_s4_hi") && motion_kind != hash40("attack_s4_lw") && 
                        motion_kind != hash40("attack_lw4") && motion_kind != hash40("attack_hi4") && motion_kind != hash40("special_s") && motion_kind != hash40("special_n") {
                            ItemModule::have_item(boma, smash::app::ItemKind(*ITEM_KIND_ROBOTGYRO), 0, 0, false, false);
                            GYRO_LIFE[ENTRY_ID] = 0;
                        }
                    }
                }
            }
        }

        // DANGER
        if situation_kind == *SITUATION_KIND_AIR {
            if STALL_TIMER[ENTRY_ID] == 900 {
                let dumb = Vector3f{x:0.0,y:10.0,z:0.0};
                EffectModule::req_follow(boma, Hash40::new("sys_flies_up"), Hash40::new("top"), &dumb, &dumb, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                SoundModule::play_se(boma, Hash40::new("se_common_spirits_machstamp_landing"), true, false, false, false, enSEType(0));
                STALL_TIMER[ENTRY_ID] = 901;
            }
            else if STALL_TIMER[ENTRY_ID] == 901 {
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
        }
        if DamageModule::reaction(boma, 0) > 1.0 { 
            STALL_TIMER[ENTRY_ID] = 0;
        }
    }
}

// ON START
pub unsafe extern "C" fn robot_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        PUMMEL_AMOUNT[ENTRY_ID] = 0;
        GYRO_LIFE[ENTRY_ID] = 0;
        STALL_TIMER[ENTRY_ID] = 0;
    }
}

pub fn install() {
    Agent::new("robot")
        .on_line(Main, robot_frame)
        .on_start(robot_start)
        .install();
}