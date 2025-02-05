use super::*;

// OPFF
pub unsafe extern "C" fn robot_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let motion_kind = MotionModule::motion_kind(boma);

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
    }
}

// ON START
pub unsafe extern "C" fn robot_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        PUMMEL_AMOUNT[ENTRY_ID] = 0;
        GYRO_LIFE[ENTRY_ID] = 0;
    }
}

pub fn install() {
    Agent::new("robot")
        .on_line(Main, robot_frame)
        .on_start(robot_start)
        .install();
}