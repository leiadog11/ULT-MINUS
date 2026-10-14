use super::*;

// SMASH ATTACKS
unsafe extern "C" fn purin_smash_attacks_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    CHARGE_MUL[ENTRY_ID] = 0.0;
    PostureModule::set_scale(fighter.module_accessor, 1.0, false);

    return 0.into();
}

// NEUTRAL B
unsafe extern "C" fn purin_specialn_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    macros::STOP_SE(fighter, Hash40::new("se_purin_special_n01"));

    return 0.into();
}

pub fn install() {
    Agent::new("purin")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW4, purin_smash_attacks_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, purin_smash_attacks_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_S4, purin_smash_attacks_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, purin_smash_attacks_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_SPECIAL_N, purin_specialn_check_damage)
        
    .install();
}