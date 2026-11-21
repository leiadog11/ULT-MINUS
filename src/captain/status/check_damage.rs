use super::*;

// FORWARD SMASH
unsafe extern "C" fn captain_attacks4_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_FALCONPUNCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    return 0.into();
}

// ZOOM IN
unsafe extern "C" fn captain_zoom_in_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    SlowModule::clear_whole(fighter.module_accessor);
    CameraModule::reset_all(fighter.module_accessor);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_boss_finishhit"), false, false);
    macros::CAM_ZOOM_OUT(fighter);

    return 0.into();
}

pub fn install() {
    Agent::new("captain")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_S4, captain_attacks4_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_HI3, captain_zoom_in_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_100, captain_zoom_in_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_AIR, captain_zoom_in_check_damage)

    .install();
}