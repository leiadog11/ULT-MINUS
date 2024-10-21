use super::*;

// OPFF
pub unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        if WorkModule::get_int(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW) > 3 {
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW);
        }

        let mut opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
        if opponent_boma == fighter.module_accessor {
            opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        }

        let b1x = PostureModule::pos_x(fighter.module_accessor);
        let b1y = PostureModule::pos_y(fighter.module_accessor);
        let b2x = PostureModule::pos_x(opponent_boma);
        let b2y = PostureModule::pos_y(opponent_boma);
        
        // distance formula
        let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
        let d = dSquared.sqrt();
        
        if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON || status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
            if d < 21.0 {
                macros::SLOW_OPPONENT(fighter, 2.0, 1.0);
                DamageModule::add_damage(opponent_boma, 0.05, 0);
            }      
        }
        else {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_timer"), false, true);
        }

        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_s_r") ||
           MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_s_l") {
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