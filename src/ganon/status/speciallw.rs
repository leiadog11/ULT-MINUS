use super::*;

///////////DOWN B 

// PRE
unsafe extern "C" fn ganon_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
      
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn ganon_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_SP_BRAKE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_CLIFF_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_FALL_ONOFF);

    let ganon_kick_hit_reduct_count = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("ganon_kick_hit_reduct_count")); 
    WorkModule::set_int(fighter.module_accessor, ganon_kick_hit_reduct_count, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_REDUCTION_LEFT);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_GANON_KICK_SPEED_COEFFICIENT);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_END_SITUATION);

    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);

        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);

        let lr = PostureModule::lr(fighter.module_accessor); 
        WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_GANON_KICK_START_LR);

        let ganon_kick_speed_coeff = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("ganon_kick_speed_coeff")); 
        WorkModule::set_float(fighter.module_accessor, ganon_kick_speed_coeff, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_speciallw_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {  
    if MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw") {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE.into(), false.into());
            return 1.into();
        }
    }

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) &&
       MotionModule::motion_kind(fighter.module_accessor) != hash40("special_air_lw") {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 1.into();
    }
    
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION); 
    
    if start_situation != *SITUATION_KIND_GROUND {
        let is_touch = GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_DOWN).try_into().unwrap()); 
    
        if is_touch {
        let curr_frame = MotionModule::frame(fighter.module_accessor); 
            if curr_frame != 2.0 {
              WorkModule::set_int(fighter.module_accessor, *FIGHTER_GANON_KICK_END_SITUATION_AG, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_END_SITUATION);
              fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
              return 1.into(); 
            }
          return 0.into();
        }
    }
    else {
        let is_touch = GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT).try_into().unwrap()); 
        if is_touch {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK) {
                fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END.into(), false.into());
                return 1.into(); 
            } 
        }

        let is_touch = GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT).try_into().unwrap()); 
        if is_touch {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK) {
                fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END.into(), false.into());
                return 1.into(); 
            }
        } 
    }
    
        if StatusModule::is_changing(fighter.module_accessor) {
          if fighter.global_table[PREV_SITUATION_KIND] != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
          }
    
          if fighter.global_table[PREV_SITUATION_KIND] == *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                return 0.into();
            } 
          }
        }
        else {
            if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
        }
    
    return 0.into();
}

// END
unsafe extern "C" fn ganon_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {  
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); 
    WorkModule::set_float(fighter.module_accessor, sum_speed_x, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_GANON_KICK_SPEED);
    return 0.into();
}

///////// DOWN B CONTINUE

// PRE
unsafe extern "C" fn ganon_speciallw_continue_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
      
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn ganon_speciallw_continue_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hold"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_speciallw_continue_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_speciallw_continue_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE.into(), false.into());
        return 1.into();
    }

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 1.into();
    }

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        return 1.into();
    }

    let ypos = ControlModule::get_stick_y(fighter.module_accessor);
    
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR &&
    ypos < -0.5 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn ganon_speciallw_continue_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("ganon")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_speciallw_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_speciallw_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, ganon_speciallw_end)
        
        .status(Pre, FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE, ganon_speciallw_continue_pre)
        .status(Main, FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE, ganon_speciallw_continue_main)
        .status(End, FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_CONTINUE, ganon_speciallw_continue_end)
        .install();
}

