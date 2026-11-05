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

    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    SHRINK_TIME[ENTRY_ID] = 30;

    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::clear_speed_all(fighter.module_accessor);
    clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
    clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GROUND_MOVEMENT);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_shrink_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn mario_shrink_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    SHRINK_TIME[ENTRY_ID] -= 1;

    if SHRINK_TIME[ENTRY_ID] <= 0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn mario_shrink_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::CAM_ZOOM_OUT(fighter);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);

    return 0.into();
}

pub fn install() {
    Agent::new("mario")
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_SHRINK, mario_shrink_pre)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_SHRINK, mario_shrink_main)
        .status(End, FIGHTER_MARIO_STATUS_KIND_SHRINK, mario_shrink_end)
    .install();
}