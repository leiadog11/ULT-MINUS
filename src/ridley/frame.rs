use super::*;

// OPFF
pub unsafe extern "C" fn ridley_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);
        let motion = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let posx = PostureModule::pos_x(boma);
        let lr = PostureModule::lr(boma);

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

        // REMOVE AURA ON DEATH
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
            AURA[ENTRY_ID] = false;
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
    }
}

// ON START
pub unsafe extern "C" fn ridley_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        
    }
}

pub fn install() {
    Agent::new("ridley")
        .on_line(Main, ridley_frame)
        .on_start(ridley_start)
        .install();
}