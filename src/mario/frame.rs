use super::*;

// OPFF
pub unsafe extern "C" fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        let posy = PostureModule::pos_y(fighter.module_accessor);

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
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn mario_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ICEBALL);
    }
}

pub fn install() {
    Agent::new("mario")
        .on_line(Main, mario_frame)
        .on_start(mario_start)
        .install();
}