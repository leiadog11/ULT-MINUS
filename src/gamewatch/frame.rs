use super::*;

// OPFF
pub unsafe extern "C" fn gamewatch_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let frame = MotionModule::frame(fighter.module_accessor);
        let facing = PostureModule::lr(fighter.module_accessor);
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("attack_lw3"){
            if frame == 6.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::set_rate(fighter.module_accessor, 0.0);
                }
                else {
                    MotionModule::set_rate(fighter.module_accessor, 1.0);
                }
            }
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 0.6, y: PostureModule::pos_y(fighter.module_accessor)});
            }

            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 0.6, y: PostureModule::pos_y(fighter.module_accessor)});
            }
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