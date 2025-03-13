use super::*;

// OPFF
pub unsafe extern "C" fn purin_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let posx = PostureModule::pos_x(boma);
        let posy = PostureModule::pos_y(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let status_kind = StatusModule::status_kind(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true);
        }

        // SMASH ATTACK CHARGE FLOAT FOR DOWN SMASH AND FORWARD SMASH
        if motion_kind == hash40("attack_s4_hold") || motion_kind == hash40("attack_lw4_hold") {
            if CHARGE_MUL[ENTRY_ID] < 5.0 {
                CHARGE_MUL[ENTRY_ID] += 0.05;
            }
        }

        // RESET SCALE
        if motion_kind != hash40("attack_lw4") {
            PostureModule::set_scale(boma, 1.0, false);
        }

        // RESET CHARGE MUL ON HIT
        if DamageModule::reaction(boma, 0) > 1.0 { 
            CHARGE_MUL[ENTRY_ID] = 0.0;
            macros::STOP_SE(fighter, Hash40::new("se_purin_special_n01"));
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
        if motion_kind == hash40("special_hi_r") || motion_kind == hash40("special_hi_l") || 
        motion_kind == hash40("special_air_hi_r") || motion_kind == hash40("special_air_hi_l") {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            PostureModule::set_pos_2d(boma, &Vector2f {x: posx, y: posy + 0.1});

            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) ||
               ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) ||
               ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                CancelModule::enable_cancel(boma);
            }
        }

        // SLEEP TALK
        if motion_kind == hash40("special_lw_r") || motion_kind == hash40("special_lw_l") ||
        motion_kind == hash40("special_air_lw_r") || motion_kind == hash40("special_air_lw_l") {
            if MotionModule::frame(boma) >= 40.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    let rand = smash::app::sv_math::rand(hash40("fighter"), 11) as u64;
                    if rand == 1 {
                        MotionModule::change_motion(boma, Hash40::new("attack_s3_s"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 2 {
                        MotionModule::change_motion(boma, Hash40::new("attack_lw3"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 3 {
                        MotionModule::change_motion(boma, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 4 {
                        MotionModule::change_motion(boma, Hash40::new("attack_hi3"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 5 {
                        MotionModule::change_motion(boma, Hash40::new("attack_s4_s"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 6 {
                        MotionModule::change_motion(boma, Hash40::new("attack_lw4"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 7 {
                        MotionModule::change_motion(boma, Hash40::new("attack_hi4"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 8 {
                        MotionModule::change_motion(boma, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 9 {
                        MotionModule::change_motion(boma, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 10 {
                        MotionModule::change_motion(boma, Hash40::new("special_hi_r"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if rand == 11 {
                        MotionModule::change_motion(boma, Hash40::new("special_lw_r"), 0.0, 1.0, false, 0.0, false, false);
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
pub unsafe extern "C" fn purin_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        CHARGE_MUL[ENTRY_ID] = 0.0;
        METRONOME[ENTRY_ID] = 0;
        STALL_TIMER[ENTRY_ID] = 0
    }
}

pub fn install() {
    Agent::new("purin")
        .on_line(Main, purin_frame)
        .on_start(purin_start)
        .install();
}