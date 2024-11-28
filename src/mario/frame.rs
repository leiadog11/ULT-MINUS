use super::*;

// OPFF
pub unsafe extern "C" fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        let posy = PostureModule::pos_y(fighter.module_accessor);

        if motion_kind == hash40("attack_s4_hold") {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_OFF, true);
            }
        }
        if motion_kind == hash40("attack_lw4") {
            macros::SET_SPEED_EX(fighter, 0.8, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
}

// ON START
pub unsafe extern "C" fn mario_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("mario")
        .on_line(Main, mario_frame)
        .on_start(mario_start)
        .install();
}