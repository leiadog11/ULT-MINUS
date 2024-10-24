use super::*;

//------------------SPECIAL HI--------------------

// PRE
unsafe extern "C" fn captain_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), 
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}


// MAIN
unsafe extern "C" fn captain_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("Up Smash Amount START: {}", WorkModule::get_int(fighter.module_accessor, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_AMOUNT));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);

    fighter.sub_shift_status_main(L2CValue::Ptr(captain_specialhi_main_loop as *const () as _))
}


//MAIN LOOP
unsafe extern "C" fn captain_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_DEC_COOLDOWN) <= 0 {
        WorkModule::set_int(fighter.module_accessor, 20, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_DEC_COOLDOWN);
        WorkModule::dec_int(fighter.module_accessor, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_AMOUNT);
    }
    if MotionModule::frame(fighter.module_accessor) == 2.0 {
        let x_vel = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let y_vel = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let lr = PostureModule::lr(fighter.module_accessor);
        macros::SET_SPEED_EX(fighter, -0.55, 0.35, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        return 0.into();
    }
    if MotionModule::frame(fighter.module_accessor) >= 3.0 {
        if MotionModule::frame(fighter.module_accessor) == 12.0 { 
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_UP_SPECIAL_AMOUNT) <= 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 1.into();
            }
        }
        return 0.into();
    }
    return 0.into();
}

// END
unsafe extern "C" fn captain_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("captain")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, captain_specialhi_pre)

        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, captain_specialhi_main)

        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, captain_specialhi_end)

        .install();
}