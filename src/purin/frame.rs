use super::*;

// OPFF
pub unsafe extern "C" fn purin_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let posx = PostureModule::pos_x(boma);
        let posy = PostureModule::pos_y(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let motion_kind = MotionModule::motion_kind(boma);

        // SMASH ATTACK CHARGE FLOAT FOR DOWN SMASH AND FORWARD SMASH
        if motion_kind == hash40("attack_s4_hold") || motion_kind == hash40("attack_lw4_hold") {
            if CHARGE_MUL[ENTRY_ID] < 6.0 {
                CHARGE_MUL[ENTRY_ID] += 0.1;
            }
        }

        // MOVE ON DOWN TAUNT
        if motion_kind == hash40("appeal_lw_r") || motion_kind == hash40("appeal_lw_l") {
            // RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: posx + 0.8, y: posy});
            }

            // LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: posx - 0.8, y: posy});
            }

            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(boma);
            }
        }

        // RISE ON UP B + UP B CANCEL
        if METRONOME[ENTRY_ID] != 7 {
            if motion_kind == hash40("special_hi_r") || motion_kind == hash40("special_hi_l") || 
            motion_kind == hash40("special_air_hi_r") || motion_kind == hash40("special_air_hi_l") {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                PostureModule::set_pos_2d(boma, &Vector2f {x: posx, y: posy + 0.1});

                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) ||
                ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) ||
                ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    CancelModule::enable_cancel(boma);
                }
            }
        }

        // SLEEP TALK
        if motion_kind == hash40("special_lw_r") || motion_kind == hash40("special_lw_l") ||
        motion_kind == hash40("special_air_lw_r") || motion_kind == hash40("special_air_lw_l") {
            if MotionModule::frame(boma) >= 40.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    let rand = smash::app::sv_math::rand(hash40("fighter"), 11) as u64;
                    if rand == 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                    } else if rand == 1 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
                    } else if rand == 2 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
                    } else if rand == 3 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
                    } else if rand == 4 {
                        CHARGE_MUL[ENTRY_ID] = 3.0;
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
                    } else if rand == 5 {
                        CHARGE_MUL[ENTRY_ID] = 3.0;
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
                    } else if rand == 6 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);
                    } else if rand == 7 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
                    } else if rand == 8 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
                    } else if rand == 9 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
                    } else if rand == 10 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_APPEAL, true);
                    }
                }   
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn purin_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        CHARGE_MUL[ENTRY_ID] = 0.0;
        METRONOME[ENTRY_ID] = 0;
    }
}

pub fn install() {
    Agent::new("purin")
        .on_line(Main, purin_frame)
        .on_start(purin_start)
    .install();
}