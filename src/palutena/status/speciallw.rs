use super::*;

//--------------------------SPECIALLW---------------------------

// PRE
unsafe extern "C" fn palutena_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        SituationKind(*SITUATION_KIND_NONE), 
        *FIGHTER_KINETIC_TYPE_UNIQ, 
        *GROUND_CORRECT_KIND_KEEP as u32, 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        0, 
        0, 
        0, 
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

// INIT
unsafe extern "C" fn palutena_speciallw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PLANTED) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_ANCHOR_TP);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), false.into());
            return 1.into();
        }
    }
    return 0.into();
}

// MAIN
unsafe extern "C" fn palutena_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_speciallw_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn palutena_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) { 
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    return 0.into();
}

// END
unsafe extern "C" fn palutena_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}


pub fn install() {
    Agent::new("palutena")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_pre)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_end)
        .install();
}