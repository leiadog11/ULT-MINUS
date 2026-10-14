use super::*;

// TAUNT
unsafe extern "C" fn mario_taunt_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("block"), false);

    return 0.into();
}

// DOWN B
unsafe extern "C" fn mario_speciallw_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    return 0.into();
}

// SHRINK
unsafe extern "C" fn mario_shrink_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    macros::CAM_ZOOM_OUT(fighter);

    return 0.into();
}

pub fn install() {
    Agent::new("mario")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_APPEAL, mario_taunt_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_speciallw_check_damage)

        .status(CheckDamage, FIGHTER_MARIO_STATUS_KIND_SHRINK, mario_shrink_check_damage)
        
    .install();
}