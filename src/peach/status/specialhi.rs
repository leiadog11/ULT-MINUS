use super::*;

// -------- UP B --------

// PRE
unsafe extern "C" fn peach_specicalhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_STATUS_ATTR_START_TURN).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn peach_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);

    fighter.fastshift(L2CValue::Ptr(peach_specialhi_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn peach_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN.into(), false.into());
        return 1.into();
    }
    else {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn peach_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// -------- UP B OPEN --------

// PRE
unsafe extern "C" fn peach_specicalhi_open_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_STATUS_ATTR_START_TURN).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn peach_specialhi_open_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_open"), 0.0, 1.0, false, 0.0, false, false);

    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

    fighter.fastshift(L2CValue::Ptr(peach_specialhi_open_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn peach_specialhi_open_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.1,
        0.0
    );


    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn peach_specialhi_open_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// -------- UP B ASCEND --------

// PRE
unsafe extern "C" fn peach_specicalhi_ascend_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_STATUS_ATTR_START_TURN).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn peach_specialhi_ascend_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);

    fighter.fastshift(L2CValue::Ptr(peach_specialhi_ascend_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn peach_specialhi_ascend_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    fighter.sub_transition_group_check_air_cliff();

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    let jump_speed = 1.25;
    let forward_speed = 0.5;

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        forward_speed * lr,
        0.0
    );

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        jump_speed,
        0.0
    );

    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn peach_specialhi_ascend_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("peach")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, peach_specicalhi_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, peach_specicalhi_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, peach_specicalhi_end)

        .status(Pre, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN, peach_specicalhi_open_pre)
        .status(Main, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN, peach_specicalhi_open_main)
        .status(End, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN, peach_specicalhi_open_end)

        .status(Pre, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND, peach_specicalhi_ascend_pre)
        .status(Main, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND, peach_specicalhi_ascend_main)
        .status(End, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND, peach_specicalhi_ascend_end)

        .install();
}