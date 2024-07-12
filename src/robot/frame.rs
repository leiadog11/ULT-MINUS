use super::*;

// OPFF
pub unsafe extern "C" fn robot_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let mut opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1)); //opp boma, this if statement checks for P1 slot or P2 slot
        if opponent_boma == fighter.module_accessor {
            opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0)); 
        }

        if ItemModule::is_have_item(fighter.module_accessor, 0)  { //if have gyro
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) || 
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                WorkModule::set_int(fighter.module_accessor, 300, FIGHTER_ROB_INSTANCE_WORK_INT_GYRO_LIFE);
            }
        }
        else {
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_ROB_INSTANCE_WORK_INT_GYRO_LIFE) > 0 {
                WorkModule::dec_int(fighter.module_accessor, FIGHTER_ROB_INSTANCE_WORK_INT_GYRO_LIFE); //dec_count
            }
            if DamageModule::reaction(opponent_boma, 0) > 1.0 && WorkModule::get_int(fighter.module_accessor, FIGHTER_ROB_INSTANCE_WORK_INT_GYRO_LIFE) > 0 { 
                if motion_kind != hash40("attack_air_b") && motion_kind != hash40("attack_air_f") && motion_kind != hash40("attack_air_n") &&  motion_kind != hash40("attack_air_lw") && 
                motion_kind != hash40("attack_air_hi") && motion_kind != hash40("attack_11") && motion_kind != hash40("attack_12") && motion_kind != hash40("attack_13") &&
                motion_kind != hash40("attack_dash") && motion_kind != hash40("catch_attack") && motion_kind != hash40("cliff_attack") && motion_kind != hash40("slip_attack") &&
                motion_kind != hash40("throw_b") && motion_kind != hash40("throw_hi") && motion_kind != hash40("throw_f") && motion_kind != hash40("throw_lw") &&
                motion_kind != hash40("attack_s3_s") && motion_kind != hash40("attack_s3_hi") && motion_kind != hash40("attack_s3_lw") && motion_kind != hash40("attack_lw3") && 
                motion_kind != hash40("attack_hi3") && motion_kind != hash40("attack_s4_s") && motion_kind != hash40("attack_s4_hi") && motion_kind != hash40("attack_s4_lw") && 
                motion_kind != hash40("attack_lw4") && motion_kind != hash40("attack_hi4") && motion_kind != hash40("special_s") && motion_kind != hash40("special_n") {
                    ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_ROBOTGYRO), 0, 0, false, false);
                    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_ROB_INSTANCE_WORK_INT_GYRO_LIFE);
                }
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn robot_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("robot")
        .on_line(Main, robot_frame)
        .on_start(robot_start)
        .install();
}