use super::*;

//////////// DOWN B

// PRE
unsafe extern "C" fn pit_specicallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
unsafe extern "C" fn pit_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    if SPECIAL_LW_TIMER[ENTRY_ID] == 1200 || !SPECIAL_LW_ACTIVE[ENTRY_ID] {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
        SPECIAL_LW_ACTIVE[ENTRY_ID] = true;
    }

    fighter.fastshift(L2CValue::Ptr(pit_speciallw_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pit_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    let frame = MotionModule::frame(figther.module_accessor);

    if frame == 1 {
        if SPECIAL_LW_TIMER[ENTRY_ID] < 1200 {
            SPECIAL_LW_ACTIVE[ENTRY_ID] = false;
            fighter.change_status(FIGHTER_STATUS_KIND_THROWN.into(), false.into());
            return 1.into();
        }
    }

    // i think i need this
    if MotionModule::is_end(fighter.module_accessor) { 
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

// END
unsafe extern "C" fn pit_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

//////////// DOWN B END

// PRE
unsafe extern "C" fn pit_specicallw_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_LW_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_LW_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_LW_END_FLOAT,
        (*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_VISIBILITY)
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
unsafe extern "C" fn pit_speciallw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_end"), 0.0, 1.0, false, 0.0, false, false);

    fighter.fastshift(L2CValue::Ptr(pit_speciallw_end_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pit_speciallw_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    // i think i need this
    if MotionModule::is_end(fighter.module_accessor) { 
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }
    
    return 0.into();
}

// END
unsafe extern "C" fn pit_speciallw_end_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("pit")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, pit_specicallw_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, pit_speciallw_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, pit_speciallw_end)

        .status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, pit_specicallw_end_pre)
        .status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, pit_speciallw_end_main)
        .status(End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, pit_speciallw_end_end)
        
        .install();
}