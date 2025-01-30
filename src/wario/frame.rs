use super::*;

// OPFF
pub unsafe extern "C" fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let pos_x = PostureModule::pos_x(boma);
        let pos_y = PostureModule::pos_y(boma);
        let lr = PostureModule::lr(boma);
        let mut max_speed = 0.0;
        let damage = DamageModule::damage(boma, 0);

        // CAMERA ZOOM OUT ON END OF UP B
        if MotionModule::motion_kind(boma) == hash40("special_hi_jump") {
            if MotionModule::is_end(boma) {
                SlowModule::clear_whole(boma);
                CameraModule::reset_all(boma);
                EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
        }

        // CAMERA ZOOM OUT ON LEDGE GRAB AND DAMAGE
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CLIFF_CATCH ||
        DamageModule::reaction(boma, 0) > 1.0 {
            SlowModule::clear_whole(boma);
            CameraModule::reset_all(boma);
            EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }

        // RESET DOWN SMASH COUNT
        if DamageModule::reaction(boma, 0) > 1.0 {
            DOWN_SMASH_AMOUNT[ENTRY_ID] = 0;
        }

        // MOVING ON DOWN SMASH
        if MotionModule::motion_kind(boma) == hash40("attack_lw4") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x + 0.55, y: pos_y});
            }
            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x - 0.55, y: pos_y});
            }
        }

        // MOVING ON BACK THROW
        if MotionModule::motion_kind(boma) == hash40("throw_b") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x + 1.45, y: pos_y});
            }
            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x - 1.45, y: pos_y});
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
        if damage >= 150.0 {
            max_speed = 0.0
        }   

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            if xpos < -0.5 && lr == -1.0 { //left if facing left
                if WECTOR[ENTRY_ID] < max_speed {
                    WECTOR[ENTRY_ID] += 0.15;
                    PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x - WECTOR[ENTRY_ID], y: pos_y});
                }
            }
            else if xpos > 0.5 && lr == 1.0 { //right if facing right
                if WECTOR[ENTRY_ID] < max_speed {
                    WECTOR[ENTRY_ID] += 0.15;
                    PostureModule::set_pos_2d(boma, &Vector2f {x: pos_x + WECTOR[ENTRY_ID], y: pos_y});
                }
            }
        }
        else {
            WECTOR[ENTRY_ID] = 0.0;
        }
      
        // JUMP CANCEL DOWN AIR
        if MotionModule::motion_kind(boma) == hash40("attack_air_lw") {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(boma);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn wario_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        
    }
}

pub fn install() {
    Agent::new("wario")
        .on_line(Main, wario_frame)
        .on_start(wario_start)
        .install();
}