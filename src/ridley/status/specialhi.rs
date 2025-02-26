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
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI).try_into().unwrap(),
        0
    );
      
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
    let lr = PostureModule::lr(fighter.module_accessor);
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
        //up
        if stick_choice == 0 {
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -90.0, y: 0.0, z: 0.0}, 0);
            macros::SET_SPEED_EX(fighter, x_vel, y_vel + 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //right
        else if stick_choice == 1 {
            if lr == -1.0 {
                macros::REVERSE_LR(fighter);
            }
            macros::SET_SPEED_EX(fighter, x_vel + 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //up right
        else if stick_choice == 2 {
            if lr == -1.0 {
                macros::REVERSE_LR(fighter);
            }
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -45.0, y: 0.0, z: 0.0}, 0);
            macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //up left
        else if stick_choice == 3 {
            if lr == 1.0 {
                macros::REVERSE_LR(fighter);
            }
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: -45.0, y: 0.0, z: 0.0}, 0);
            macros::SET_SPEED_EX(fighter, -x_vel + 1.3, y_vel + 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //left
        else if stick_choice == 4 {
            if lr == 1.0 {
                macros::REVERSE_LR(fighter);
            }
            macros::SET_SPEED_EX(fighter, -x_vel + 2.0, y_vel, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //down left
        else if stick_choice == 5 {
            if lr == 1.0 {
                macros::REVERSE_LR(fighter);
            }
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 45.0, y: 0.0, z: 0.0}, 0);
            macros::SET_SPEED_EX(fighter, -x_vel + 1.3, x_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //down
        else if stick_choice == 6 {
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0);
            macros::SET_SPEED_EX(fighter, x_vel, y_vel - 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

        //down right
        else if stick_choice == 7 {
            if lr == -1.0 {
                macros::REVERSE_LR(fighter);
            }
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 45.0, y: 0.0, z: 0.0}, 0);
            macros::SET_SPEED_EX(fighter, x_vel + 1.3, y_vel - 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        return 0.into();
    }
    if MotionModule::frame(fighter.module_accessor) >= 6.0 {

        if MotionModule::frame(fighter.module_accessor) >= 20.0 { 
            // JUMP CANCEL
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
                return 1.into();
            }
            // SHIELD CANCEL
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), false.into());
                return 1.into();
            }
            // ATTACK CANCEL
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), false.into());
                return 1.into();
            }
        }

        // LANDING
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }

        if MotionModule::is_end(fighter.module_accessor) {
            PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
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

pub fn install() {
    Agent::new("ridley")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_start_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_start_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, ridley_specialhi_start_end)
        .install();
}