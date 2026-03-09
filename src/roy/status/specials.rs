use super::*;

// -------- SIDE B --------

// PRE
unsafe extern "C" fn roy_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
      
    return 0.into();
}

// INIT
unsafe extern "C" fn roy_specials_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

// EXEC
unsafe extern "C" fn roy_specials_exec(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// MAIN
unsafe extern "C" fn roy_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);

    fighter.fastshift(L2CValue::Ptr(roy_specials_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn roy_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
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
unsafe extern "C" fn roy_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("roy")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, roy_specials_pre)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, roy_specials_init)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, roy_specials_exec)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, roy_specials_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, roy_specials_end)

        .install();
}