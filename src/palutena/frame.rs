use super::*;

// OPFF
pub unsafe extern "C" fn palutena_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        // CLEAR WINGS AND CHARGE MULT ON HIT
        if DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            WorkModule::set_float(fighter.module_accessor, 1.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
        }

        /*
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_DIVE) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        */

        // CHARGE DOWN SMASH MULT
        if motion_kind == hash40("attack_lw4_hold") {
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL) < 4.0 {
                WorkModule::add_float(fighter.module_accessor, 0.075, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
            }
        }

        // MEGA LASER CHARGE AURA
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE) >= 600 {
            //white aura effect
        }

        // STOP CHARGE SOUND!
        if motion_kind != hash40("special_n_charge") {
            macros::STOP_SE(fighter, Hash40::new("se_palutena_attack100"));
            macros::STOP_SE(fighter, Hash40::new("se_palutena_special_n01"));
        }

        if motion_kind != hash40("special_n_shoot") {
            macros::STOP_SE(fighter, Hash40::new("se_palutena_final03"));
        }
    }
}

// ON START
pub unsafe extern "C" fn palutena_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::set_float(fighter.module_accessor, 1.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE);
    }
}

pub fn install() {
    Agent::new("palutena")
        .on_line(Main, palutena_frame)
        .on_start(palutena_start)
        .install();
}