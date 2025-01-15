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

    fighter.sub_shift_status_main(L2CValue::Ptr(link_specialairhistart_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn link_specialairhistart_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    // RISE WITH WIND
    macros::SET_SPEED_EX(fighter, x_vel, y_vel + 3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

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

    // GLIDE
    if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel + 0.15, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, x_vel - 0.15, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }

    // UNEQUIP GLIDER WITH A, B, OR SHIELD/DODGE
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) ||
    ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) ||
    ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP.into(), false.into());
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

//-------------------SPECIAL AIR HI UNEQUIP------------------------

// PRE
unsafe extern "C" fn link_specialairhiunequip_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    return 0.into();
}

// INIT
unsafe extern "C" fn link_specialairhiunequip_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// MAIN
unsafe extern "C" fn link_specialairhiunequip_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_unequip"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(link_specialairhiunequip_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn link_specialairhiunequip_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    // CHANGE TO FALL WHEN ANIM IS OVER
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn link_specialairhiunequip_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

//-------------------SPECIAL AIR HI EQUIP------------------------

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
    return 0.into();
}

// INIT
unsafe extern "C" fn link_specialairhiequip_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// MAIN
unsafe extern "C" fn link_specialairhiequip_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_equip"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(link_specialairhiequip_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn link_specialairhiequip_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    // CHANGE TO GLIDE WHEN ANIM IS OVER
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_GLIDE.into(), false.into());
        return 1.into();
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
    return 0.into();
}

// INIT
unsafe extern "C" fn link_specialairhilanding_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// MAIN
unsafe extern "C" fn link_specialairhilanding_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_landing"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    fighter.sub_shift_status_main(L2CValue::Ptr(link_specialairhilanding_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn link_specialairhilanding_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

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

        .status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP, link_specialairhiunequip_pre)
        .status(Init, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP, link_specialairhiunequip_init)
        .status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP, link_specialairhiunequip_main)
        .status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP, link_specialairhiunequip_end)

        .status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP, link_specialairhiequip_pre)
        .status(Init, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP, link_specialairhiequip_init)
        .status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP, link_specialairhiequip_main)
        .status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_UNEQUIP, link_specialairhiequip_end)

        .status(Pre, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING, link_specialairhilanding_pre)
        .status(Init, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING, link_specialairhilanding_init)
        .status(Main, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING, link_specialairhilanding_main)
        .status(End, FIGHTER_LINK_STATUS_KIND_SPECIAL_AIR_HI_LANDING, link_specialairhilanding_end)

        .install();
}