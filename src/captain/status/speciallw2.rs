use super::*;

//------------------SPECIAL LW 2--------------------

// PRE
unsafe extern "C" fn captain_speciallw2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_FALL, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

// MAIN
unsafe extern "C" fn captain_speciallw2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw2"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);

    fighter.sub_shift_status_main(L2CValue::Ptr(captain_speciallw2_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn captain_speciallw2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let opponent_bomas = get_opponent_bomas(fighter.module_accessor);
    if frame >= 4.0 && frame <= 24.0 {
        for opponent_boma in opponent_bomas.iter() { 
            if AttackModule::is_infliction_status(*opponent_boma, *COLLISION_KIND_MASK_HIT) {
                println!("FALCON DEFLECT!");
                macros::EFFECT(fighter, Hash40::new("sys_passive"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
                //SoundModule::play_se(fighter.module_accessor, Hash40::new("se_item_teamhealfield_recover"), true, false, false, false, enSEType(0));
                StatusModule::change_status_request_from_script(*opponent_boma, *FIGHTER_STATUS_KIND_CATCH_CUT, true);
                //fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4.into(), false.into());
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s"), 0.0, 1.0, false, 0.0, false, false);
                return 1.into();
            }
        }
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            KICK[get_entry_id(fighter.module_accessor)] = true;
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
            return 1.into();
        }
        return 0.into();
    }
    if frame > 24.0 {
        HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }   
    }
    return 0.into();
}

// END
unsafe extern "C" fn captain_speciallw2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub fn install() {
    Agent::new("captain")
        .status(Pre, FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW2, captain_speciallw2_pre)

        .status(Main, FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW2, captain_speciallw2_main)

        .status(End, FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW2, captain_speciallw2_end)

        .install();
}

