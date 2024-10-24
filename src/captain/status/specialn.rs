use super::*;

//--------------------SPECIAL N------------------------

// PRE
unsafe extern "C" fn captain_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_FALL, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}



// INIT
unsafe extern "C" fn captain_specialn_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    
    return 0.into();
}

// MAIN
unsafe extern "C" fn captain_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    fighter.sub_shift_status_main(L2CValue::Ptr(captain_specialn_main_loop as *const () as _))
}

//MAIN LOOP
unsafe extern "C" fn captain_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }   
    }
    return 0.into();
}

// END
unsafe extern "C" fn captain_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("captain")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_specialn_pre)

        //.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_specialn_init)

        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_specialn_main)

        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, captain_specialn_end)

        .install();
}

