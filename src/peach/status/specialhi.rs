use super::*;

// -------- UP B --------

// PRE
unsafe extern "C" fn peach_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_START_TURN).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn peach_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);

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
unsafe extern "C" fn peach_specialhi_open_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_START_TURN).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn peach_specialhi_open_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_open"), 0.0, 1.0, false, 0.0, false, false);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);

    fighter.fastshift(L2CValue::Ptr(peach_specialhi_open_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn peach_specialhi_open_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -0.2,
        0.0
    );

    // TRANSITION TO LANDING ON GROUND
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND { 
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_GLIDE.into(), false.into());
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
unsafe extern "C" fn peach_specialhi_ascend_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
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
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
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

    let lr = PostureModule::lr(fighter.module_accessor);
    let jump_speed = 2.5;
    let forward_speed = 0.5;
    let frame = MotionModule::frame(fighter.module_accessor);

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
        jump_speed - (frame * 0.06),
        0.0
    );

    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn peach_specialhi_ascend_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// -------- UP B GLIDE --------

// PRE
unsafe extern "C" fn peach_specialhi_glide_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn peach_specialhi_glide_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fall"), 0.0, 1.0, false, 0.0, false, false);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);

    KineticModule::clear_speed_all(fighter.module_accessor);

    fighter.fastshift(L2CValue::Ptr(peach_specialhi_glide_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn peach_specialhi_glide_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    fighter.sub_transition_group_check_air_cliff();

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    let ypos = ControlModule::get_stick_y(fighter.module_accessor);

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -0.2,
        0.0
    );

    // TRANSITION TO ASCEND ON UP B
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && ypos > 0.5 { 
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND.into(), false.into());
        return 1.into();
    }

    // CLOSE UMBRELLA ON A
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        return 1.into();
    }

    // JUMP CANCEL
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) { 
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 1.into();
    }

    // TRANSITION TO LANDING ON GROUND
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND { 
        fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn peach_specialhi_glide_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("peach")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, peach_specialhi_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, peach_specialhi_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, peach_specialhi_end)

        .status(Pre, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN, peach_specialhi_open_pre)
        .status(Main, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN, peach_specialhi_open_main)
        .status(End, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_OPEN, peach_specialhi_open_end)

        .status(Pre, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND, peach_specialhi_ascend_pre)
        .status(Main, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND, peach_specialhi_ascend_main)
        .status(End, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_ASCEND, peach_specialhi_ascend_end)

        .status(Pre, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_GLIDE, peach_specialhi_glide_pre)
        .status(Main, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_GLIDE, peach_specialhi_glide_main)
        .status(End, FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_GLIDE, peach_specialhi_glide_end)

        .install();
}