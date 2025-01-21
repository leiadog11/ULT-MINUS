use super::*;

//-------------------SPECIAL AIR HI START------------------------

// PRE
unsafe extern "C" fn link_specialairhistart_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), 
        true, 
        0, 
        0, 
        0, 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    return 0.into();
}

// INIT
unsafe extern "C" fn link_specialairhistart_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// MAIN
unsafe extern "C" fn link_specialairhistart_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_EQUIPPED);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_USED);

    fighter.sub_shift_status_main(L2CValue::Ptr(link_specialairhistart_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn link_specialairhistart_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);

    fighter.sub_transition_group_check_air_cliff();

    // RISE WITH WIND
    if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel, y_vel + 0.01, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, -x_vel, y_vel + 0.01, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }

    if MotionModule::frame(fighter.module_accessor) == 12.0 {
        HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }

    // CHANGE TO GLIDE WHEN ANIM IS OVER
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn link_specialairhistart_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//-------------------SPECIAL AIR HI GLIDE------------------------

// PRE
unsafe extern "C" fn link_specialairhiglide_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), 
        true, 
        0, 
        0, 
        0, 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    return 0.into();
}

// INIT
unsafe extern "C" fn link_specialairhiglide_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// MAIN
unsafe extern "C" fn link_specialairhiglide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_glide"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(link_specialairhiglide_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn link_specialairhiglide_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);

    fighter.sub_transition_group_check_air_cliff();

    // GLIDE
    if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel + 0.02, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, -x_vel + 0.02, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }

    // UNEQUIP GLIDER WITH A, B, JUMP, OR SHIELD/DODGE
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) ||
    ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) ||
    ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) ||
    ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP.into(), false.into());
        return 1.into();
    }

    // LAND IF TOUCHING GROUND
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn link_specialairhiglide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//-------------------SPECIAL AIR HI EQUIP/UNEQUIP------------------------

// PRE
unsafe extern "C" fn link_specialairhiequip_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), 
        true, 
        0, 
        0, 
        0, 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    return 0.into();
}

// INIT
unsafe extern "C" fn link_specialairhiequip_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// MAIN
unsafe extern "C" fn link_specialairhiequip_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_EQUIPPED) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_unequip"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_equip"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(link_specialairhiequip_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn link_specialairhiequip_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);

    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_EQUIPPED) { 
        if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel, -0.01, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, -x_vel, -0.01, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }
    }

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_EQUIPPED) { 
        // CHANGE TO FALL WHEN ANIM IS OVER - UNEQUIP
        if MotionModule::is_end(fighter.module_accessor) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_EQUIPPED);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    else {
        // CHANGE TO GLIDE WHEN ANIM IS OVER - EQUIP
        if MotionModule::is_end(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_EQUIPPED);
            fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

// END
unsafe extern "C" fn link_specialairhiequip_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//-------------------SPECIAL AIR HI LANDING------------------------

// PRE
unsafe extern "C" fn link_specialairhilanding_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), 
        true, 
        0, 
        0, 
        0, 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    return 0.into();
}

// INIT
unsafe extern "C" fn link_specialairhilanding_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// MAIN
unsafe extern "C" fn link_specialairhilanding_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_landing"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    fighter.sub_shift_status_main(L2CValue::Ptr(link_specialairhilanding_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn link_specialairhilanding_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);

    if MotionModule::frame(fighter.module_accessor) == 29.0 { 
        macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    if MotionModule::frame(fighter.module_accessor) >= 30.0 {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) ||
        ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) ||
        ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) ||
        ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) || 
        stick_x >= 0.3 || stick_x <= 0.3 { 
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }

    // CHANGE TO WAIT WHEN ANIM IS OVER
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn link_specialairhilanding_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    return 0.into();
}

pub fn install() {
    Agent::new("link")
        .status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START, link_specialairhistart_pre)
        .status(Init, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START, link_specialairhistart_init)
        .status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START, link_specialairhistart_main)
        .status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_START, link_specialairhistart_end)
    
        .status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE, link_specialairhiglide_pre)
        .status(Init, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE, link_specialairhiglide_init)
        .status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE, link_specialairhiglide_main)
        .status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE, link_specialairhiglide_end)

        .status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, link_specialairhiequip_pre)
        .status(Init, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, link_specialairhiequip_init)
        .status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, link_specialairhiequip_main)
        .status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, link_specialairhiequip_end)

        .status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING, link_specialairhilanding_pre)
        .status(Init, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING, link_specialairhilanding_init)
        .status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING, link_specialairhilanding_main)
        .status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING, link_specialairhilanding_end)

        .install();
}