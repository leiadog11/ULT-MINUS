use super::*;

// OPFF
pub unsafe extern "C" fn purin_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let posx = PostureModule::pos_x(fighter.module_accessor);
        let posy = PostureModule::pos_y(fighter.module_accessor);
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        //SMASH ATTACK CHARGE FLOAT FOR DOWN SMASH AND FORWARD SMASH
        if motion_kind == hash40("attack_s4_hold") || motion_kind == hash40("attack_lw4_hold") {
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_PURIN_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL) < 5.0 {
                WorkModule::add_float(fighter.module_accessor, 0.05, FIGHTER_PURIN_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
            }
        }

        if motion_kind != hash40("attack_lw4") {
            PostureModule::set_scale(fighter.module_accessor, 1.0, false);
        }

        if DamageModule::reaction(fighter.module_accessor, 0) > 1.0 { 
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_PURIN_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL);
        }

        //MOVE ON DOWN TAUNT
        if motion_kind == hash40("appeal_lw_r") || motion_kind == hash40("appeal_lw_l") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 0.8, y: posy});
            }

            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 0.8, y: posy});
            }

            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }

        //RISE ON UP B
        if motion_kind == hash40("special_hi_r") || motion_kind == hash40("special_hi_l") || 
        motion_kind == hash40("special_air_hi_r") || motion_kind == hash40("special_air_hi_l") {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx, y: posy + 0.1});
        }

        //SLEEP TALK
        if motion_kind == hash40("special_lw_r") || motion_kind == hash40("special_lw_l") ||
        motion_kind == hash40("special_air_lw_r") || motion_kind == hash40("special_air_lw_l") {
            if MotionModule::frame(fighter.module_accessor) >= 40.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    let rand = smash::app::sv_math::rand(hash40("fighter"), 11) as u64;
                    if rand == 1 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s3_s"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 2 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw3"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 3 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 4 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi3"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 5 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 6 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 7 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi4"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 8 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 9 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 10 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_r"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 11 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_r"), 0.0, 1.0, false, 0.0, false, false);
                    }
                }   
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn purin_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("purin")
        .on_line(Main, purin_frame)
        .on_start(purin_start)
        .install();
}