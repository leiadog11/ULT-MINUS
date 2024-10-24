use super::*;

// OPFF
unsafe extern "C" fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        let is_touch = GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT).try_into().unwrap());
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        // MOVE ON RAPID JAB
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_100") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 1.5, y: PostureModule::pos_y(fighter.module_accessor)});
            }
            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 1.5, y: PostureModule::pos_y(fighter.module_accessor)});
            }
        }

        // DASH CANCEL DASH ATTACK
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_dash") {
            if MotionModule::frame(fighter.module_accessor) > 9.0 && MotionModule::frame(fighter.module_accessor) < 19.0 {
                if xpos > 0.5 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
                if xpos < -0.5 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                    
                }
            }
        }

        // INVINCIBILITY ON DOWN SMASH
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw_4") {
            if MotionModule::frame(fighter.module_accessor) > 1.0 {
                macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
            }
        }

        // DASH AND JUMP CANCEL LASER
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if MotionModule::frame(fighter.module_accessor) > 6.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) &&
                StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR { 
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
                }
                else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) &&
                StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
                    macros::SET_SPEED_EX(fighter, sum_speed_x, -1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }

                if xpos > 0.5 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
                    if lr == -1.0 {
                        macros::REVERSE_LR(fighter);
                    }
                }
                else if xpos < -0.5 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DASH, true);
                    if lr == 1.0 {
                        macros::REVERSE_LR(fighter);
                    }
                }
                else if ypos < -0.5 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        }

        // AIR DODGE CANCEL UP B START
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if MotionModule::frame(fighter.module_accessor) > 1.0 && MotionModule::frame(fighter.module_accessor) < 36.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }

        // JUMP CANCEL SIDE B
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
            }
            else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) { //short hop?
                if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
                    macros::SET_SPEED_EX(fighter, sum_speed_x, -1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            }
        }

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_f") && is_touch {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f_beak_bonk"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

// ON START
pub unsafe extern "C" fn falco_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("falco")
        .on_line(Main, falco_frame)
        .on_start(falco_start)
        .install();
}
