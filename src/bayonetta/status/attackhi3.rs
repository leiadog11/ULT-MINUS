use super::*;

//-------------------UP TILT--------------------

// PRE
unsafe extern "C" fn bayonetta_attackhi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    return 0.into();
}

// INIT
unsafe extern "C" fn bayonetta_attackhi3_init(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }

    return 0.into();
}

// MAIN
unsafe extern "C" fn bayonetta_attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi3"), 0.0, 1.0, false, 0.0, false, false);

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
      
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attackhi3_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn bayonetta_attackhi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::frame(fighter.module_accessor) > 15.0 {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
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

// END
unsafe extern "C" fn bayonetta_attackhi3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    return 0.into();
}

pub fn install() {
    Agent::new("bayonetta")
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_HI3, bayonetta_attackhi3_pre)
        .status(Init, *FIGHTER_STATUS_KIND_ATTACK_HI3, bayonetta_attackhi3_init)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_HI3, bayonetta_attackhi3_main)
        .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI3, bayonetta_attackhi3_end)
    .install();
}