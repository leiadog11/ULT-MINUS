use super::*;

// OPFF
pub unsafe extern "C" fn ridley_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);

        // ACTIVATE AURA
        if DamageModule::damage(fighter.module_accessor, 0) >= 100.0 && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_AURA) { 
            let dumb = Vector3f{x:0.0,y:10.0,z:0.0};
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_boss_core_hit"), true, false, false, false, enSEType(0));
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_fire_m"), true, false, false, false, enSEType(0));
            let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_special_defense_up"), Hash40::new("top"), &dumb, &dumb, 3.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(fighter.module_accessor, effect, 0.9, 0.0, 0.5);
            EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_AURA);
        }

        // REMOVE AURA ON DEATH
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_AURA);
        }

        // TAUNT IDLE
        if motion == hash40("appeal_lw_r") || motion == hash40("appeal_lw_l") {
            if frame >= 25.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait_taunt"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion == hash40("wait_taunt") {
            if xpos > 0.0 || xpos < 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("walk_taunt"), 0.0, 1.0, false, 0.0, false, false);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }

        // TAUNT WALK
        if motion == hash40("walk_taunt") {
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 0.8, y: PostureModule::pos_y(fighter.module_accessor)});
            }
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 0.8, y: PostureModule::pos_y(fighter.module_accessor)});
            }
            if xpos == 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait_taunt"), 0.0, 1.0, false, 0.0, false, false);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn ridley_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLAG_AURA);
    }
}

pub fn install() {
    Agent::new("ridley")
        .on_line(Main, ridley_frame)
        .on_start(ridley_start)
        .install();
}