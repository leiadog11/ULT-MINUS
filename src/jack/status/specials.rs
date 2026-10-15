use super::*;
use smash::app::KineticEnergy;

//------------------SPECIAL S--------------------

// PRE
unsafe extern "C" fn jack_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), 
        false, 
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

// INIT
unsafe extern "C" fn jack_specials_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        let start_spd_x_mul = 0.8;
        sv_kinetic_energy!(mul_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, start_spd_x_mul, 1.0);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    return 0.into();
}

// MAIN
pub unsafe extern "C" fn jack_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(jack_specials_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn jack_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_JACK_STATUS_KIND_SPECIAL_S_DASH.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn jack_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

//------------------SPECIAL S DASH--------------------

// PRE
unsafe extern "C" fn jack_specials_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), 
        false, 
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

// INIT
unsafe extern "C" fn jack_specials_dash_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        let start_spd_x_mul = 0.8;
        sv_kinetic_energy!(mul_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, start_spd_x_mul, 1.0);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    return 0.into();
}

// MAIN
pub unsafe extern "C" fn jack_specials_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_dash"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(jack_specials_dash_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn jack_specials_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.sub_transition_group_check_air_cliff();

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 3.5 * lr, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.025, 0.0);

    // JUMP CANCEL ON OR AFTER FRAME 8
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && MotionModule::frame(fighter.module_accessor) >= 8.0 {
        EffectModule::clear_all_after_image(fighter.module_accessor, 2);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("jack_final_speedline"), false, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("tex_jack_sword1"), false, true);
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {  
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        } else {
            fighter.change_status(FIGHTER_JACK_STATUS_KIND_SPECIAL_S_SLIDE.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

// END
unsafe extern "C" fn jack_specials_dash_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

//------------------SPECIAL S SLIDE--------------------

// PRE
unsafe extern "C" fn jack_specials_slide_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_GROUND), 
        *FIGHTER_KINETIC_TYPE_GROUND_STOP, 
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, 
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        false, 
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

// MAIN
pub unsafe extern "C" fn jack_specials_slide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_slide"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(jack_specials_slide_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn jack_specials_slide_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn jack_specials_slide_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("jack")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, jack_specials_pre)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, jack_specials_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, jack_specials_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, jack_specials_end)

        .status(Pre, FIGHTER_JACK_STATUS_KIND_SPECIAL_S_DASH, jack_specials_dash_pre)
        .status(Init, FIGHTER_JACK_STATUS_KIND_SPECIAL_S_DASH, jack_specials_dash_init)
        .status(Main, FIGHTER_JACK_STATUS_KIND_SPECIAL_S_DASH, jack_specials_dash_main)
        .status(End, FIGHTER_JACK_STATUS_KIND_SPECIAL_S_DASH, jack_specials_dash_end)

        .status(Pre, FIGHTER_JACK_STATUS_KIND_SPECIAL_S_SLIDE, jack_specials_slide_pre)
        .status(Main, FIGHTER_JACK_STATUS_KIND_SPECIAL_S_SLIDE, jack_specials_slide_main)
        .status(End, FIGHTER_JACK_STATUS_KIND_SPECIAL_S_SLIDE, jack_specials_slide_end)

    .install();
}