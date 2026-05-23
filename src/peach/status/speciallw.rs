use super::*;

// -------- DOWN B --------

unsafe extern "C" fn peach_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn peach_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue { 
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_ITEM_NO_COUNT);

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND { 
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);

        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04) + -1);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(peach_speciallw_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn peach_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    fighter.sub_transition_group_check_air_cliff();

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND { 
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }

    if MotionModule::is_end(fighter.module_accessor) || CancelModule::is_enable_cancel(fighter.module_accessor) { 
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

pub fn install() {
    Agent::new("peach")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, peach_speciallw_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, peach_speciallw_main)

        .install();
}