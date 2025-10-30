use super::*;

// -------- DOWN B --------

// PRE
unsafe extern "C" fn roy_specicallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_START_TURN).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn roy_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);

    fighter.fastshift(L2CValue::Ptr(roy_speciallw_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn roy_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
        fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_DIVE.into(), false.into());
        return 1.into();
    }
    else {
        fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_ROLL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn roy_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// -------- DOWN B ROLL --------

// PRE
unsafe extern "C" fn roy_specicallw_roll_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn roy_speciallw_roll_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_roll"), 0.0, 1.0, false, 0.0, false, false);

    fighter.fastshift(L2CValue::Ptr(roy_speciallw_roll_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn roy_speciallw_roll_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn roy_speciallw_roll_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// -------- DOWN B DIVE --------

// PRE
unsafe extern "C" fn roy_specicallw_dive_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn roy_speciallw_dive_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_dive"), 0.0, 1.0, false, 0.0, false, false);

    fighter.fastshift(L2CValue::Ptr(roy_speciallw_dive_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn roy_speciallw_dive_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 

    // TOUCH GROUND TRANSITION TO LANDING STATUS
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into());
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) { 
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

// END
unsafe extern "C" fn roy_speciallw_dive_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// -------- DOWN B LANDING --------

// PRE
unsafe extern "C" fn roy_specicallw_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_GROUND).try_into().unwrap(),
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn roy_speciallw_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_landing"), 0.0, 1.0, false, 0.0, false, false); // add to motion list and make animation

    fighter.fastshift(L2CValue::Ptr(roy_speciallw_landing_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn roy_speciallw_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn roy_speciallw_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("roy")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, roy_specicallw_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, roy_speciallw_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, roy_speciallw_end)

        .status(Pre, FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_ROLL, roy_specicallw_roll_pre)
        .status(Main, FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_ROLL, roy_speciallw_roll_main)
        .status(End, FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_ROLL, roy_speciallw_roll_end)

        .status(Pre, FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_DIVE, roy_specicallw_dive_pre)
        .status(Main, FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_DIVE, roy_speciallw_dive_main)
        .status(End, FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_DIVE, roy_speciallw_dive_end)

        .status(Pre, FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_LANDING, roy_specicallw_landing_pre)
        .status(Main, FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_LANDING, roy_speciallw_landing_main)
        .status(End, FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_LANDING, roy_speciallw_landing_end)

        .install();
}