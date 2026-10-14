use super::*;

// WINGS CHECK
unsafe extern "C" fn palutena_wings_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    return 0.into();
}

// DOWN SMASH
unsafe extern "C" fn palutena_attacklw4_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    CHARGE_MUL[ENTRY_ID] = 1.0;

    return 0.into();
}

pub fn install() {
    Agent::new("palutena")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_S3, palutena_wings_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW3, palutena_wings_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW4, palutena_attacklw4_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, palutena_attacklw4_check_damage)
        
    .install();
}