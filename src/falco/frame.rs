use super::*;

// OPFF
unsafe extern "C" fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let pos_x = PostureModule::pos_x(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let lr = PostureModule::lr(boma);
        let is_touch = GroundModule::is_touch(boma, (*GROUND_TOUCH_FLAG_RIGHT).try_into().unwrap());
        let sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let ENTRY_ID = get_entry_id(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH { 
            GroundModule::set_collidable(boma, true);
        }

        // MOVE ON RAPID JAB
        if motion_kind == hash40("attack_100") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x + 0.8, y: PostureModule::pos_y(boma)});
            }
            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x - 0.8, y: PostureModule::pos_y(boma)});
            }
        }

        // DASH CANCEL DASH ATTACK
        if motion_kind == hash40("attack_dash") {
            if frame > 9.0 && frame < 19.0 {
                if xpos > 0.5 {
                    CancelModule::enable_cancel(boma);
                }
                if xpos < -0.5 {
                    CancelModule::enable_cancel(boma);
                    
                }
            }
        }

        // INVINCIBILITY ON DOWN SMASH
        if motion_kind == hash40("attack_lw_4") {
            if frame > 1.0 {
                macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
            }
        }

        // DASH AND JUMP CANCEL LASER
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if frame > 6.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) &&
                situation_kind != *SITUATION_KIND_AIR { 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) &&
                situation_kind != *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
                    macros::SET_SPEED_EX(fighter, sum_speed_x, -1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }

                if xpos > 0.5 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
                    if lr == -1.0 {
                        macros::REVERSE_LR(fighter);
                    }
                }
                else if xpos < -0.5 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
                    if lr == 1.0 {
                        macros::REVERSE_LR(fighter);
                    }
                }
                else if ypos < -0.5 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        }

        // AIR DODGE CANCEL UP B START
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame > 1.0 && frame < 36.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }

        // JUMP CANCEL SIDE B
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
            }
            else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) { //short hop?
                if situation_kind != *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
                    macros::SET_SPEED_EX(fighter, sum_speed_x, -1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            }
        }

        // BEAK BONK
        if motion_kind == hash40("attack_air_f") && is_touch {
            MotionModule::change_motion(boma, Hash40::new("attack_air_f_beak_bonk"), 0.0, 1.0, false, 0.0, false, false);
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
pub unsafe extern "C" fn falco_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        STALL_TIMER[ENTRY_ID] = 0;
    }
}

pub fn install() {
    Agent::new("falco")
        .on_line(Main, falco_frame)
        .on_start(falco_start)
        .install();
}
