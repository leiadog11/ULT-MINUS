use super::*;

// OPFF
pub unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        static mut opp_bomas: Option<Vec<*mut BattleObjectModuleAccessor>> = None;

        if WorkModule::get_int(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW) > 3 {
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW);
        }

        // GET OPPONENT BOMAS ON START
        if motion_kind == hash40("entry_r") || motion_kind == hash40("entry_l") { 
            if MotionModule::is_end(fighter.module_accessor) {
                OPPONENT_BOMAS = Some(get_opponent_bomas(fighter));
                opp_bomas = OPPONENT_BOMAS.clone();
            }
        }

        let b1x = PostureModule::pos_x(fighter.module_accessor);
        let b1y = PostureModule::pos_y(fighter.module_accessor);

        if let Some(ref opponent_bomas) = opp_bomas {
            for (index, &boma_ptr) in opponent_bomas.iter().enumerate() {
                let b2x = PostureModule::pos_x(boma_ptr);
                let b2y = PostureModule::pos_y(boma_ptr);   
    
                // distance formula
                let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
                let d = dSquared.sqrt();
    
                if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON || status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
                    if d < 21.0 {
                        //macros::SLOW_OPPONENT(fighter, 2.0, 1.0);
                        DamageModule::add_damage(boma_ptr, 0.05, 0);
                        SlowModule::set_whole(boma_ptr, 2, 1);
                    }
                }
                else {
                    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_timer"), false, true);
                }
            }
        } 

        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        if motion_kind == hash40("appeal_s_r") || motion_kind == hash40("appeal_s_l") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx + 0.6, y: PostureModule::pos_y(fighter.module_accessor)});
            }

            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: posx - 0.6, y: PostureModule::pos_y(fighter.module_accessor)});
            }

            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) ||
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            if ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
            }
    
            if ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_l"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn luigi_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_MISFIRE_SPECIAL_N);
    }
}

pub fn install() {
    Agent::new("luigi")
        .on_line(Main, luigi_frame)
        .on_start(luigi_start)
        .install();
}