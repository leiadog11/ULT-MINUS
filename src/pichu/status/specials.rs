use super::*;

//////////// SIDE B

// INIT
unsafe extern "C" fn pichu_specials_init(fighter: &mut L2CFighterCommon) -> L2CValue { 
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    return 0.into();
}

// PRE
unsafe extern "C" fn pichu_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_START_TURN).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn pichu_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);

    //smash input check?
    //angle check?

    fighter.fastshift(L2CValue::Ptr(pichu_specials_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pichu_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status((*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD).into(), false.into());

        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn pichu_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

///////////// SIDE B HOLD

// INIT
unsafe extern "C" fn pichu_specials_hold_init(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// PRE
unsafe extern "C" fn pichu_specials_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
      );
      
      FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn pichu_specials_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_hold"), 0.0, 1.0, false, 0.0, false, false);

    fighter.fastshift(L2CValue::Ptr(pichu_specials_hold_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pichu_specials_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    macros::SET_SPEED_EX(fighter, 5.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) ||
    ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());

        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        fighter.change_status((*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END).into(), false.into());

        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn pichu_specials_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

/////////////// SIDE B END

// INIT
unsafe extern "C" fn pichu_specials_end_init(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// PRE
unsafe extern "C" fn pichu_specials_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
      );
      
      FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S).try_into().unwrap(),
        0
      );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn pichu_specials_end_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);

    fighter.fastshift(L2CValue::Ptr(pichu_specials_end_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pichu_specials_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());

        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn pichu_specials_end_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    return 0.into();
}

pub fn install() {
    Agent::new("pichu")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_specials_init)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_specials_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_specials_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, pichu_specials_end)

        .status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pichu_specials_hold_init)
        .status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pichu_specials_hold_pre)
        .status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pichu_specials_hold_main)
        .status(End, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, pichu_specials_hold_end)

        .status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pichu_specials_end_init)
        .status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pichu_specials_end_pre)
        .status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pichu_specials_end_main)
        .status(End, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, pichu_specials_end_end)
        
        .install();
}