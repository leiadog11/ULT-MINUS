use super::*;

//--------------------------DOWN SPECIAL---------------------------

// PRE
unsafe extern "C" fn mario_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

// MAIN
unsafe extern "C" fn mario_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, false, -1);
    }   
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        //KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        //fighter.set_situation(SITUATION_KIND_AIR.into());
        //GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, WEAPON_MARIO_PUMP_STATUS_KIND_BOOST, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_charge"), 0.0, 1.0, false, 0.0, false, false);
    }
    
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn mario_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND { 
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, true);
    }

    if MotionModule::frame(fighter.module_accessor) == 1.0 { 
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, WEAPON_MARIO_PUMP_STATUS_KIND_BOOST, ArticleOperationTarget(0));
    }

    // CHANGE FLUDD TO END
    if MotionModule::frame(fighter.module_accessor) == 29.0 { 
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, *WEAPON_MARIO_PUMP_STATUS_KIND_END, ArticleOperationTarget(0));
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

// END
unsafe extern "C" fn mario_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}


pub fn install() {
    Agent::new("mario")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_speciallw_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_speciallw_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_speciallw_end)
        .install();
}