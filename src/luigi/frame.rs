use super::*;

// OPFF
pub unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe { 
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        let posx = PostureModule::pos_x(fighter.module_accessor);

        // DOWN TILT COUNTER
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW) > 3 {
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_ATTACK_LW);
        }

        // GET OPPONENT BOMAS ON SHIELD
        if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON || status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
            let entry_count = lua_bind::FighterManager::entry_count(singletons::FighterManager());
            let entry_count_usize = entry_count as usize;
            let mut opponent_bomas: Vec<*mut BattleObjectModuleAccessor> = Vec::with_capacity(entry_count_usize);
            let mut boma_counter = 0;
    
            for _ in 0..entry_count_usize { 
                let curr_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(boma_counter));
                if curr_boma == fighter.module_accessor {
                }
                else {
                    opponent_bomas.push(sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(boma_counter)));
                }
                boma_counter += 1;
            }

            let neg_zone = WorkModule::get_float(fighter.module_accessor, WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_FLOAT_NEG_ZONE);

            if neg_zone < 1.7 {
                WorkModule::add_float(fighter.module_accessor, 0.05, WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_FLOAT_NEG_ZONE);
                macros::EFFECT(fighter, Hash40::new("luigi_negative_zone"), Hash40::new("top"), 0.0, 7.5, 0.0, 0, 0, 0, neg_zone, 0, 0, 0, 0, 0, 0, true);
            }

            let b1x = PostureModule::pos_x(fighter.module_accessor);
            let b1y = PostureModule::pos_y(fighter.module_accessor);

            // NEGATIVE ZONE
            for &boma_ptr in &opponent_bomas {
                let b2x = PostureModule::pos_x(boma_ptr);
                let b2y = PostureModule::pos_y(boma_ptr);   
        
                // distance formula
                let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
                let d = dSquared.sqrt();
        
                if d < 22.0 {
                    //macros::SLOW_OPPONENT(fighter, 2.0, 1.0);
                    DamageModule::add_damage(boma_ptr, 0.05, 0);
                    SlowModule::set_whole(boma_ptr, 2, 1);
                }
            } 
        }
        else {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("luigi_negative_zone"), false, true);
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_luigi_negative_zone"), 0);
            WorkModule::set_float(fighter.module_accessor, 0.1, WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_FLOAT_NEG_ZONE);
        }

        // MOVE DURING SIDE TAUNT
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

        // USE SIDE TAUNT IN AIR
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
        WorkModule::set_float(fighter.module_accessor, 0.1, WEAPON_LUIGI_FIREBALL_INSTANCE_WORK_FLOAT_NEG_ZONE);
    }
}

pub fn install() {
    Agent::new("luigi")
        .on_line(Main, luigi_frame)
        .on_start(luigi_start)
        .install();
}