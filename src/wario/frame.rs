use super::*;

// OPFF
pub unsafe extern "C" fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        let mut max_speed = 0.0;
        let damage = DamageModule::damage(fighter.module_accessor, 0);

        // CAMERA ZOOM OUT ON END OF UP B
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi_jump") {
            if MotionModule::is_end(fighter.module_accessor) {
                SlowModule::clear_whole(fighter.module_accessor);
                CameraModule::reset_all(fighter.module_accessor);
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
        }

        // CAMERA ZOOM OUT ON LEDGE GRAB AND DAMAGE
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_CATCH ||
        DamageModule::reaction(fighter.module_accessor, 0) > 1.0 {
            SlowModule::clear_whole(fighter.module_accessor);
            CameraModule::reset_all(fighter.module_accessor);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }

        // MOVING ON DOWN SMASH
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw4") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 0.5, y: PostureModule::pos_y(fighter.module_accessor)});
            }
            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 0.5, y: PostureModule::pos_y(fighter.module_accessor)});
            }
        }

        // MOVING ON BACK THROW
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_b") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 1.45, y: PostureModule::pos_y(fighter.module_accessor)});
            }
            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 1.45, y: PostureModule::pos_y(fighter.module_accessor)});
            }
        }

        // WECTORING
        if damage > 10.0 && damage < 30.0 {
            max_speed = 3.0
        }
        if damage > 30.0 && damage < 60.0 {
            max_speed = 6.0
        }
        if damage > 60.0 && damage < 90.0 {
            max_speed = 8.0
        }
        if damage > 90.0 {
            max_speed = 10.0
        }        

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            if xpos < -0.5 && lr == -1.0 { //left if facing left
                if WorkModule::get_float(fighter.module_accessor, FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_WECTOR) < max_speed {
                    WorkModule::add_float(fighter.module_accessor, 0.15, FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_WECTOR);
                    PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - WorkModule::get_float(fighter.module_accessor, FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_WECTOR), y: PostureModule::pos_y(fighter.module_accessor)});
                }
            }
            else if xpos > 0.5 && lr == 1.0 { //right if facing right
                if WorkModule::get_float(fighter.module_accessor, FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_WECTOR) < max_speed {
                    WorkModule::add_float(fighter.module_accessor, 0.15, FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_WECTOR);
                    PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + WorkModule::get_float(fighter.module_accessor, FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_WECTOR), y: PostureModule::pos_y(fighter.module_accessor)});
                }
            }
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_WECTOR);
        }
      
        // JUMP CANCEL DOWN AIR
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn wario_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_WARIO_INSTANCE_WORK_ID_INT_ATTACK_LW4);
    }
}

pub fn install() {
    Agent::new("wario")
        .on_line(Main, wario_frame)
        .on_start(wario_start)
        .install();
}