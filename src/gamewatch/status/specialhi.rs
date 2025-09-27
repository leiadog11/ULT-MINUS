use super::*;


// -------- SPECIAL HI START --------

// PRE
unsafe extern "C" fn gamewatch_specialhi_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_AIR).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
      
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI  | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn gamewatch_specialhi_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(gamewatch_specialhi_start_main_loop as *const () as _));
    return 0.into();
}


// MAIN LOOP
unsafe extern "C" fn gamewatch_specialhi_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
	let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let mut x_control = 0.0;

    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        return 1.into();
    }
    else {
        fighter.sub_transition_group_check_air_cliff();

        //up
        if stick_x == 0.0 && stick_y > 0.5 {
            x_control = 0.0;
        }

        //right
        if stick_x > 0.5 && stick_y == 0.0 {
            x_control = 0.25;
        }

        //up right
        if stick_x > 0.3 && stick_y > 0.3 {
            x_control = 0.125;
        }

        //up left
        if stick_x < -0.3 && stick_y > 0.3 {
            x_control = 0.125;
        }

        //left
        if stick_x < -0.5 && stick_y == 0.0 {
            x_control = 0.25;
        }

        //down left
        if stick_x < -0.3 && stick_y < -0.3 {
            x_control = 0.125;
        }

        //down
        if stick_x == 0.0 && stick_y < -0.5 {
            x_control = 0.0;
        }

        //down right
        if stick_x > 0.3 && stick_y < -0.3 {
            x_control = 0.125;
        }
            // rise in air
        macros::SET_SPEED_EX(fighter, (x_vel + x_control) * lr, 4.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    return 0.into();
}

// END
unsafe extern "C" fn gamewatch_specialhi_start_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

// -------- SPECIAL HI FALL --------

// PRE
unsafe extern "C" fn gamewatch_specialhi_fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_AIR).try_into().unwrap(),
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
    );
      
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn gamewatch_specialhi_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(gamewatch_specialhi_fall_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn gamewatch_specialhi_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    // FALL UNTIL TOUCHING GROUND
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND { 
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    } 
    else {
        MotionModule::set_rate(fighter.module_accessor, 0.0);
        macros::SET_SPEED_EX(fighter, 0.0, y_vel - 15.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    fighter.sub_transition_group_check_air_cliff();
    
    return 0.into();
}

// END
unsafe extern "C" fn gamewatch_specialhi_fall_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));

    return 0.into();
}

pub fn install() {
    Agent::new("gamewatch")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, gamewatch_specialhi_start_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, gamewatch_specialhi_start_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, gamewatch_specialhi_start_end)
        
        .status(Pre, *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_FALL, gamewatch_specialhi_fall_pre)
        .status(Main, *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_FALL, gamewatch_specialhi_fall_main)
        .status(End, *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_FALL, gamewatch_specialhi_fall_end)
        .install();
}