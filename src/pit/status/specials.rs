use super::*;

// -------- SIDE B --------

// PRE
unsafe extern "C" fn pit_specicals_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
      
    return 0.into();
}

// MAIN
unsafe extern "C" fn pit_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    // RISE SLIGHTLY FROM GROUNDED
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{ x: 0.0, y: 2.0, z: 0.0 });
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }

    fighter.fastshift(L2CValue::Ptr(pit_specials_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pit_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    let frame = MotionModule::frame(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);

    fighter.sub_transition_group_check_air_cliff();

    if frame > 6.0 && frame < 17.0 {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 2.5 * lr, 0.0);
    } else if frame >= 19.0 {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.65 * lr, 0.0);
    } else if frame > 40.0 {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND { 
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn pit_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue { 
    return 0.into();
}

pub fn install() {
    Agent::new("pit")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, pit_specicals_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, pit_specials_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, pit_specials_end)

    .install();
}