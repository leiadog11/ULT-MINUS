use super::*;

//-------------------DASH ATTACK--------------------

// PRE
unsafe extern "C" fn pacman_attackdash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
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

// MAIN
unsafe extern "C" fn pacman_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
      
    fighter.sub_shift_status_main(L2CValue::Ptr(pacman_attackdash_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pacman_attackdash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::frame(fighter.module_accessor) >= 51.0 { 
        GroundModule::set_collidable(fighter.module_accessor, true);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    } else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }

        let lr = PostureModule::lr(fighter.module_accessor);
        let forward_speed = 1.25;

        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            forward_speed * lr,
            0.0
        );

        blastzone_check(fighter, 60.0);
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
unsafe extern "C" fn pacman_attackdash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("pacman")
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_DASH, pacman_attackdash_pre)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, pacman_attackdash_main)
        .status(End, *FIGHTER_STATUS_KIND_ATTACK_DASH, pacman_attackdash_end)

        .install();
}