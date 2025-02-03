use super::*;

// OPFF
pub unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let ENTRY_ID = get_entry_id(boma);
        let status_kind = StatusModule::status_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let lr = PostureModule::lr(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let ypos = ControlModule::get_stick_y(boma);
        let posx = PostureModule::pos_x(boma);

        // NEGATIVE ZONE
        if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON || status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
            let b1x = PostureModule::pos_x(boma);
            let b1y = PostureModule::pos_y(boma);

            let opponent_bomas = get_opponent_bomas(boma);
            for opponent_boma in opponent_bomas.iter() {
                let b2x = PostureModule::pos_x(*opponent_boma);
                let b2y = PostureModule::pos_y(*opponent_boma);   
        
                // distance formula
                let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
                let d = dSquared.sqrt();
        
                if d < 22.0 {
                    //macros::SLOW_OPPONENT(fighter, 2.0, 1.0);
                    DamageModule::add_damage(*opponent_boma, 0.05, 0);
                    SlowModule::set_whole(*opponent_boma, 2, 1);
                }
            } 

            if NEG_ZONE[ENTRY_ID] < 1.7 {
                NEG_ZONE[ENTRY_ID] += 0.05;
                if (NEG_ZONE[ENTRY_ID] % 0.2 < 0.0001) || (0.2 - (NEG_ZONE[ENTRY_ID] % 0.2) < 0.0001) {
                    EffectModule::kill_kind(boma, Hash40::new("luigi_negative_zone"), false, true);
                }
                macros::EFFECT(fighter, Hash40::new("luigi_negative_zone"), Hash40::new("top"), 0.5, 7.5, 0.0, 0, 0, 0, NEG_ZONE[ENTRY_ID], 0, 0, 0, 0, 0, 0, true);
            }
        }
        else {
            EffectModule::kill_kind(boma, Hash40::new("luigi_negative_zone"), false, true);
            SoundModule::stop_se(boma, Hash40::new("se_luigi_negative_zone"), 0);
            NEG_ZONE[ENTRY_ID] = 0.1
        }

        // COLLISION FIX
        if motion_kind == hash40("attack_hi4") {
            if frame >= 25.0 {
                let opponent_bomas = get_opponent_bomas(boma);
                for opponent_boma in opponent_bomas.iter() { 
                    GroundModule::set_collidable(*opponent_boma, true);
                }
            }
        }

        // MOVE DURING SIDE TAUNT
        if motion_kind == hash40("appeal_s_r") || motion_kind == hash40("appeal_s_l") {
            //RIGHT
            if xpos > 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: posx + 0.6, y: PostureModule::pos_y(boma)});
            }

            //LEFT
            if xpos < 0.0  {
                PostureModule::set_pos_2d(boma, &Vector2f {x: posx - 0.6, y: PostureModule::pos_y(boma)});
            }

            // CANCEL IT
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                CancelModule::enable_cancel(boma);
            }
        }

        // USE SIDE TAUNT IN AIR
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR { 
            if ControlModule::check_button_release(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                MotionModule::change_motion(boma, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
            }
    
            if ControlModule::check_button_release(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                MotionModule::change_motion(boma, Hash40::new("appeal_s_l"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

// ON START
pub unsafe extern "C" fn luigi_start(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        MISFIRE_SPECIAL_N[ENTRY_ID] = false;
        MISFIRE_UP_SMASH[ENTRY_ID] = false;
        DOWN_TILT_COUNTER[ENTRY_ID] = 0;
        NEG_ZONE[ENTRY_ID] = 0.0;
    }
}

pub fn install() {
    Agent::new("luigi")
        .on_line(Main, luigi_frame)
        .on_start(luigi_start)
        .install();
}