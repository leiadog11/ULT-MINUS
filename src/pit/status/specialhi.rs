use super::*;

// -------- UP B --------

// PRE
unsafe extern "C" fn pit_specicalhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_START_TURN).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// INIT
unsafe extern "C" fn pit_specialhi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    if IS_FLIGHT[ENTRY_ID] {
        IS_FLIGHT[ENTRY_ID] = false;
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// MAIN
unsafe extern "C" fn pit_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);

    fighter.fastshift(L2CValue::Ptr(pit_specialhi_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pit_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn pit_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// -------- UP B RUSH (INFINITE FLIGHT)

// PRE
unsafe extern "C" fn pit_specicalhi_flight_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        0,
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn pit_specialhi_flight_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_flight"), 0.0, 1.0, false, 0.0, false, false);

    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    IS_FLIGHT[ENTRY_ID] = true;

    fighter.fastshift(L2CValue::Ptr(pit_specialhi_flight_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pit_specialhi_flight_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    // maybe we just make this something in frame? effect spawns in frame too? wing animation???
    return 0.into();
}

// END
unsafe extern "C" fn pit_specialhi_flight_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("pit")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, pit_specicalhi_pre)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, pit_specialhi_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, pit_specialhi_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, pit_specialhi_end)
        
        .status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, pit_specicalhi_flight_pre)
        .status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, pit_specialhi_flight_main)
        .status(End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, pit_specialhi_flight_end)
    
        .install();
}