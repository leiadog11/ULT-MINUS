use super::*;

// -------- DOWN B --------

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

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) { 
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
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, peach_speciallw_main)

        .install();
}