use super::*;


//------------------SPECIAL HI START--------------------

// PRE
unsafe extern "C" fn ridley_specialhi_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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

// EXEC
unsafe extern "C" fn ridley_specialhi_start_exec(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    if UP_B_USES[ENTRY_ID] == 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    return 0.into();
}

// MAIN
unsafe extern "C" fn ridley_specialhi_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    UP_B_USES[ENTRY_ID] -= 1;
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_specialhi_start_main_loop as *const () as _));
    return 0.into();
}

// MAIN LOOP
unsafe extern "C" fn ridley_specialhi_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
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

    if MotionModule::frame(fighter.module_accessor) == 5.0 {
        let ENTRY_ID = get_entry_id(fighter.module_accessor);
        let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let lr = PostureModule::lr(fighter.module_accessor);
        //up
        if stick_choice == 0 {
            macros::SET_SPEED_EX(fighter, x_vel, y_vel + 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -90.0, y: 0.0, z: 0.0}, 0);
            println!("UP!");
        }

        //right
        else if stick_choice == 1 {
            if lr != 1.0 {
                macros::REVERSE_LR(fighter);
            }
            macros::SET_SPEED_EX(fighter, x_vel + 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            println!("RIGHT!");
        }

        //up right
        else if stick_choice == 2 {
            if lr != 1.0 {
                macros::REVERSE_LR(fighter);
            }
            macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 45.0, y: 0.0, z: 0.0}, 0);
            println!("UP RIGHT!");
        }

        //up left
        else if stick_choice == 3 {
            if lr == 1.0 {
                macros::REVERSE_LR(fighter);
            }
            macros::SET_SPEED_EX(fighter, x_vel - 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            println!("UP LEFT!");
        }

        //left
        else if stick_choice == 4 {
            if lr == 1.0 {
                macros::REVERSE_LR(fighter);
            }
            macros::SET_SPEED_EX(fighter, x_vel - 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            println!("LEFT!");
        }

        //down left
        else if stick_choice == 5 {
            if lr == 1.0 {
                macros::REVERSE_LR(fighter);
            }
            macros::SET_SPEED_EX(fighter, x_vel - 1.3, x_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            println!("DOWN LEFT!");
        }

        //down
        else if stick_choice == 6 {
            macros::SET_SPEED_EX(fighter, x_vel, y_vel - 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -90.0, y: 0.0, z: 0.0}, 0);
            println!("DOWN!");
        }

        //down right
        else if stick_choice == 7 {
            if lr != 1.0 {
                macros::REVERSE_LR(fighter);
            }
            macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            println!("DOWN RIGHT!");
        }
        return 0.into();
    }
    if MotionModule::frame(fighter.module_accessor) >= 6.0 {
        KineticModule::resume_energy_all(fighter.module_accessor);

        // JUMP CANCEL
        if MotionModule::frame(fighter.module_accessor) >= 20.0 { 
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
                return 1.into();
            }
        }

        // LANDING
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
            return 1.into();
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
unsafe extern "C" fn ridley_specialhi_start_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
    return 0.into();
}

//------------------SPECIAL HI LANDING--------------------

// PRE
unsafe extern "C" fn ridley_specialhi_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        (*GROUND_CORRECT_KIND_KEEP).try_into().unwrap(),
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI).try_into().unwrap(),
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn ridley_specialhi_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_landing"), 0.0, 1.0, false, 0.0, false, false);

    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_specialhi_landing_main_loop as *const () as _));
    return 0.into();
}

// MAIN LOOP
unsafe extern "C" fn ridley_specialhi_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);

    if MotionModule::frame(fighter.module_accessor) == 10.0 { 
        macros::SET_SPEED_EX(fighter, (x_vel + 1.0) * lr, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    return 0.into();
}

// END
unsafe extern "C" fn ridley_specialhi_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("ridley")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_start_pre)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_start_exec)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_start_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_start_end)
        
        .status(Pre, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING, ridley_specialhi_landing_pre)
        .status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING, ridley_specialhi_landing_main)
        .status(End, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING, ridley_specialhi_landing_end)
        .install();
}