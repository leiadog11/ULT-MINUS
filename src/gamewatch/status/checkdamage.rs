use super::*;

// NEUTRAL AIR + BACK AIR
unsafe extern "C" fn gamewatch_attackairn_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_OCTOPUS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("blockm"), false);
    } else if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b") {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

    return 0.into();
}

// UP TILT
unsafe extern "C" fn gamewatch_attackhi3_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    return 0.into();
}

// DOWN SMASH
unsafe extern "C" fn gamewatch_attacklw4_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    PostureModule::set_scale(fighter.module_accessor, 1.0, false);

    return 0.into();
}

pub fn install() {
    Agent::new("gamewatch")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_AIR, gamewatch_attackairn_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_HI3, gamewatch_attackhi3_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW4, gamewatch_attacklw4_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, gamewatch_attacklw4_check_damage)
        
    .install();
}