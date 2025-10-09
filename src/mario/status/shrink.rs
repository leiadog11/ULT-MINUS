use super::*;

//--------------------------SHRINK---------------------------

// PRE
unsafe extern "C" fn mario_shrink_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        0, 
        0, 
        0, 
        0
    );

    return 0.into();
}

// MAIN
unsafe extern "C" fn mario_shrink_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shrink"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(mario_shrink_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn mario_shrink_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    SHRINK_TIME[ENTRY_ID] -= 1;

    if SHRINK_TIME[ENTRY_ID] <= 43 {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy::set_accel_x_add(fighter.lua_state_agent);
        sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
        KineticModule::clear_speed_all(fighter.module_accessor);
    }

    if SHRINK_TIME[ENTRY_ID] <= 0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn mario_shrink_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    SHRINK_TIME[ENTRY_ID] = 45;
    macros::CAM_ZOOM_OUT(fighter);
    return 0.into();
}


pub fn install() {
    Agent::new("mario")
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHRINK, mario_shrink_pre)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_SHRINK, mario_shrink_main)
        .status(End, FIGHTER_MARIO_STATUS_KIND_SHRINK, mario_shrink_end)
        .install();
}