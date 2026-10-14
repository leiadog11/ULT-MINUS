use super::*;

// CHECK DAMAGE
unsafe extern "C" fn ganon_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    return 0.into();
}

pub fn install() {
    Agent::new("ganon")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_AIR, ganon_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_S4, ganon_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, ganon_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_HI4, ganon_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, ganon_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW4, ganon_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, ganon_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_SPECIAL_N, ganon_check_damage)
        
    .install();
}