use super::*;

// OPFF
pub unsafe extern "C" fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        let posy = PostureModule::pos_y(fighter.module_accessor);
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);

        // SHIELD CANCEL FORWARD SMASH CHARGE
        if motion_kind == hash40("attack_s4_hold") {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            }
        }
        /*
        if DamageModule::reaction(fighter.module_accessor, 0) > 90.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("shrink"), 0.0, 1.0, false, 0.0, false, false);
        }
        */

        // CANCEL DASH ATTACK INTO DASH ATTACK 2
        if motion_kind == hash40("attack_dash") && frame > 5.0 { 
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("breakdance"), 0.0, 1.0, false, 0.0, false, false);
            }
        }

        // FORWARD SMASH CHARGE
        if motion_kind == hash40("attack_s4_hold") || motion_kind == hash40("attack_s4_s") || motion_kind == hash40("attack_s4_hi") || motion_kind == hash40("attack_s4_lw") {
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_SMASH_CHARGE) < 60.0 {
                WorkModule::add_float(fighter.module_accessor, 1.0, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_SMASH_CHARGE);
            }
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_SMASH_CHARGE);
        }

        // FORWARD AIR CHARGE
        if motion_kind == hash40("attack_air_f") { 
            if frame >= 6.0 && frame <= 13.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::set_rate(fighter.module_accessor, 0.0);
                    macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("arml"), 2.0, 0, -2.5, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
                    if WorkModule::get_float(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_AIR_CHARGE) < 20.0 {
                        WorkModule::add_float(fighter.module_accessor, 1.0, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_AIR_CHARGE);
                    }
                    else if WorkModule::get_float(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_AIR_CHARGE) == 20.0 {
                        macros::EFFECT(fighter, Hash40::new("sys_hit_critical"), Hash40::new("arml"), 0.0, 0.0, 0.0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                        if !SoundModule::is_playing(fighter.module_accessor, Hash40::new("se_gohoubi_bounus_add")) {
                            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_gohoubi_bounus_add"), true, false, false, false, enSEType(0));
                            WorkModule::add_float(fighter.module_accessor, 1.0, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_AIR_CHARGE);
                        }
                    }
                }
                else {
                    MotionModule::set_rate(fighter.module_accessor, 1.0);
                }
            }
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_AIR_CHARGE);
        }

        // MOVE DURING DOWN SMASH
        if motion_kind == hash40("attack_lw4") {
            if frame < 25.0 {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 0.75, y: PostureModule::pos_y(fighter.module_accessor)});
            }
            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 0.75, y: PostureModule::pos_y(fighter.module_accessor)});
            }
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn mario_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ICEBALL);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_SMASH_CHARGE);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_FORWARD_AIR_CHARGE);
    }
}

pub fn install() {
    Agent::new("mario")
        .on_line(Main, mario_frame)
        .on_start(mario_start)
        .install();
}