use super::*;

// OPFF
pub unsafe extern "C" fn gamewatch_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let frame = MotionModule::frame(fighter.module_accessor);
        if frame == 6.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                macros::SET_SPEED_EX(fighter, 0.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                MotionModule::set_rate(fighter.module_accessor, 0.0);
            }
        }
        else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }

    }
}

// ON START
pub unsafe extern "C" fn gamewatch_start(fighter: &mut L2CFighterCommon) {
    unsafe { 

    }
}

pub fn install() {
    Agent::new("gamewatch")
        .on_line(Main, gamewatch_frame)
        .on_start(gamewatch_start)
        .install();
}