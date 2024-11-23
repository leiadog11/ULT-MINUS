use super::*;

// OPFF
pub unsafe extern "C" fn palutena_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        // NO SPECIAL FALL ON UP B + EXTRA JUMP
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_DIVE) {
            if frame >= 20.0 {
                //WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_UP_B_USED);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }

        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF { 
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_UP_B_USED);
        } 

        // CLEAR WINGS AND CHARGE MULT ON HIT
        if DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            WorkModule::set_float(fighter.module_accessor, 1.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
        }

        // CHARGE DOWN SMASH MULT
        if motion_kind == hash40("attack_lw4_hold") {
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL) < 4.0 {
                WorkModule::add_float(fighter.module_accessor, 0.075, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
            }
        }

        // STOP CHARGE SOUND!
        if motion_kind != hash40("special_n_charge") && motion_kind != hash40("special_s_charge") && motion_kind != hash40("attack_hi4") {
            macros::STOP_SE(fighter, Hash40::new("se_palutena_attack100"));
            macros::STOP_SE(fighter, Hash40::new("se_palutena_special_n01"));
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("palutena_wand_light2"), false, true);
        }

        if motion_kind != hash40("special_n_shoot") {
            macros::STOP_SE(fighter, Hash40::new("se_palutena_final03"));
        }

        // TP TO ANCHOR WITH AERIAL DOWN B 
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PLANTED) && 
        WorkModule::is_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_ANCHOR_TP) {  
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && situation_kind == *SITUATION_KIND_AIR { 
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            }

            if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3 && situation_kind == *SITUATION_KIND_AIR {
                if frame >= 1.0 { 
                    GroundModule::set_collidable(fighter.module_accessor, false);
                    PostureModule::set_pos(fighter.module_accessor, &Vector3f{ x: BULLET_X_POS , y: BULLET_Y_POS, z: PostureModule::pos_z(fighter.module_accessor)});
                }
                if frame >= 2.0 { 
                    GroundModule::set_collidable(fighter.module_accessor, true);
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PLANTED);
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_ANCHOR_TP);
                }
            }
        }

        // AND MEGA LASER
        if motion_kind == hash40("special_s_shoot") && WorkModule::get_int(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE) >= 360 && frame >= 25.0 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_N_SHOOT, false);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn palutena_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::set_float(fighter.module_accessor, 1.0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PLANTED);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_ANCHOR_TP);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_UP_B_USED);
    }
}

pub fn install() {
    Agent::new("palutena")
        .on_line(Main, palutena_frame)
        .on_start(palutena_start)
        .install();
}