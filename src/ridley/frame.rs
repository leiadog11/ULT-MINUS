use super::*;

// OPFF
pub unsafe extern "C" fn ridley_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let motion = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let posx = PostureModule::pos_x(boma);
        let lr = PostureModule::lr(boma);

        // ON RESPAWN
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
            AURA[ENTRY_ID] = false;
            GroundModule::set_collidable(boma, true);
        }

        // ON GROUND
        if situation_kind == *SITUATION_KIND_GROUND || situation_kind == *SITUATION_KIND_CLIFF {
            UP_B_USES[ENTRY_ID] = 3; // UP B USES BACK
        }

        // ACTIVATE AURA
        if DamageModule::damage(boma, 0) >= 100.0 && !AURA[ENTRY_ID] { 
            let dumb = Vector3f{x:0.0,y:10.0,z:0.0};
            SoundModule::play_se(boma, Hash40::new("se_common_boss_core_hit"), true, false, false, false, enSEType(0));
            SoundModule::play_se(boma, Hash40::new("se_common_fire_m"), true, false, false, false, enSEType(0));
            let effect = EffectModule::req_follow(boma, Hash40::new("sys_special_defense_up"), Hash40::new("top"), &dumb, &dumb, 3.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(boma, effect, 0.9, 0.0, 0.5);
            EffectModule::enable_sync_init_pos_last(boma);
            AURA[ENTRY_ID] = true;
        }

        // TAUNT IDLE
        if motion == hash40("appeal_lw_r") || motion == hash40("appeal_lw_l") {
            if frame >= 25.0 {
                MotionModule::change_motion(boma, Hash40::new("wait_taunt"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("wait_taunt") {
            if xpos > 0.0 || xpos < 0.0 {
                MotionModule::change_motion(boma, Hash40::new("walk_taunt"), 0.0, 1.0, false, 0.0, false, false);
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(boma);
            }
        }

        // TAUNT WALK
        if motion == hash40("walk_taunt") {
            if xpos > 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: posx + 0.8, y: PostureModule::pos_y(boma)});
            }
            if xpos < 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: posx - 0.8, y: PostureModule::pos_y(boma)});
            }
            if xpos == 0.0 {
                MotionModule::change_motion(boma, Hash40::new("wait_taunt"), 0.0, 1.0, false, 0.0, false, false);
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(boma);
            }
        }

        // UP AIR 2
        if motion == hash40("attack_air_hi") { 
            if frame >= 11.0 && frame <= 22.0 { 
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::set_rate(boma, 0.0);
                    if UP_AIR_HOLD[ENTRY_ID] < 5 {
                        UP_AIR_HOLD[ENTRY_ID] += 1;
                    }
                }
                else {
                    MotionModule::set_rate(boma, 1.0);
                    if UP_AIR_HOLD[ENTRY_ID] >= 5 {
                        MotionModule::change_motion(boma, Hash40::new("attack_air_hi2"), 0.0, 1.0, false, 0.0, false, false);
                    }
                }
            }
        }
        else {
            UP_AIR_HOLD[ENTRY_ID] = 0;
        }

        // CANCEL SIDE B DRAG INTO GRAB
        if motion == hash40("special_s_drag_f") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(boma, Hash40::new("catch"), 0.0, 1.0, false, 0.0, false, false);
            }
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
pub unsafe extern "C" fn ridley_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        AURA[ENTRY_ID] = false;
        UP_B_USES[ENTRY_ID] = 3;
        UP_AIR_HOLD[ENTRY_ID] = 0;
        STALL_TIMER[ENTRY_ID] = 0;
    }
}

pub fn install() {
    Agent::new("ridley")
        .on_line(Main, ridley_frame)
        .on_start(ridley_start)
        .install();
}