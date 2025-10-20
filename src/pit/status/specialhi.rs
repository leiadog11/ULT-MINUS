use super::*;

// -------- UP B --------

// PRE
unsafe extern "C" fn pit_specicalhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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

// MAIN
unsafe extern "C" fn pit_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

    KineticModule::add_speed(fighter.module_accessor, &Vector3f{ x: 0.0, y: 1.0, z: 0.0 });

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
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    fighter.fastshift(L2CValue::Ptr(pit_specialhi_flight_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pit_specialhi_flight_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
	let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let flight_speed_mul = 2.0;

    fighter.sub_transition_group_check_air_cliff();

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        stick_x * flight_speed_mul,
        0.0
    );

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        stick_y * flight_speed_mul,
        0.0
    );
    
    // CANCEL WITH ATTACK OR AIR DODGE SPECIAL
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) ||
    ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) ||
    ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_b"), true, true);
        macros::STOP_SE(fighter, Hash40::new("se_pit_flight_wings"));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    // LAND IF TOUCHING GROUND
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_b"), true, true);
        macros::STOP_SE(fighter, Hash40::new("se_pit_flight_wings"));
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn pit_specialhi_flight_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("pit")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, pit_specicalhi_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, pit_specialhi_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, pit_specialhi_end)
        
        .status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, pit_specicalhi_flight_pre)
        .status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, pit_specialhi_flight_main)
        .status(End, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, pit_specialhi_flight_end)
    
        .install();
}