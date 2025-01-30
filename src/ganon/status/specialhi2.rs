use super::*;


///////////// SPECIAL HI 2 START

// PRE
unsafe extern "C" fn ganon_specialhi2_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn ganon_specialhi2_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi2_start"), 0.0, 1.0, false, 0.0, false, false);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);

    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_specialhi2_start_main_loop as *const () as _));
    return 0.into();
}


// MAIN LOOP
unsafe extern "C" fn ganon_specialhi2_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if MotionModule::is_end(fighter.module_accessor) { 
        VisibilityModule::set_whole(fighter.module_accessor, false);
        WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        GroundModule::set_passable_check(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn ganon_specialhi2_start_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}



///////// SPECIAL HI 2

// PRE
unsafe extern "C" fn ganon_specialhi2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI).try_into().unwrap(),
        (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR).try_into().unwrap(),
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn ganon_specialhi2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GROUND_CHECK[get_entry_id(fighter.module_accessor)] = false;
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi2"), 0.0, 1.0, false, 0.0, false, false);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_specialhi2_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_specialhi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
	let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let mut stick_choice = 0;

    fighter.sub_transition_group_check_air_cliff();

    //right
    if stick_x > 0.5 && stick_y == 0.0 {
        stick_choice = 1;
    }

    //up right
    if stick_x > 0.3 && stick_y > 0.3 {
        stick_choice = 2;
    }

    //up left
    if stick_x < -0.3 && stick_y > 0.3 {
        stick_choice = 3;
    }

    //left
    if stick_x < -0.5 && stick_y == 0.0 {
        stick_choice = 4;
    }

    //down left
    if stick_x < -0.3 && stick_y < -0.3 {
        stick_choice = 5;
    }

    //down
    if stick_x == 0.0 && stick_y < -0.5 {
        stick_choice = 6;
    }

    //down right
    if stick_x > 0.3 && stick_y < -0.3 {
        stick_choice = 7;
    }

    if MotionModule::frame(fighter.module_accessor) == 6.0 {
        let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let lr = PostureModule::lr(fighter.module_accessor);
        //up
        if stick_choice == 0 {
            macros::SET_SPEED_EX(fighter, x_vel, y_vel + 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //right
        else if stick_choice == 1 {
            if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel + 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, x_vel - 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }
        }

        //up right
        else if stick_choice == 2 {
            macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //up left
        else if stick_choice == 3 {
            if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel - 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }
        }

        //left
        else if stick_choice == 4 {
            if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel - 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, x_vel + 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }
        }

        //down left
        else if stick_choice == 5 {
            if lr == 1.0 { macros::SET_SPEED_EX(fighter, x_vel - 1.3, x_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); } else { macros::SET_SPEED_EX(fighter, x_vel + 1.3, x_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); }
        }

        //down
        else if stick_choice == 6 {
            macros::SET_SPEED_EX(fighter, x_vel, y_vel - 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //down right
        else if stick_choice == 7 {
            macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        return 0.into();
    }
    if MotionModule::frame(fighter.module_accessor) >= 7.0 {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        GroundModule::set_passable_check(fighter.module_accessor, false);
        KineticModule::resume_energy_all(fighter.module_accessor);

        if MotionModule::frame(fighter.module_accessor) >= 15.0 {
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }

        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        return 0.into();
    }
    return 0.into();
}

// END
unsafe extern "C" fn ganon_specialhi2_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
      }
      else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
      }
    return 0.into();
}

pub fn install() {
    Agent::new("ganon")
        .status(Pre, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START, ganon_specialhi2_start_pre)
        .status(Main, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START, ganon_specialhi2_start_main)
        .status(End, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2_START, ganon_specialhi2_start_end)
        
        .status(Pre, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2, ganon_specialhi2_pre)
        .status(Main, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2, ganon_specialhi2_main)
        .status(End, FIGHTER_GANON_STATUS_KIND_SPECIAL_HI2, ganon_specialhi2_end)
        .install();
}