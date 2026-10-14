use super::*;

// UP B
unsafe extern "C" fn wario_specialhi_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    SlowModule::clear_whole(fighter.module_accessor);
    CameraModule::reset_all(fighter.module_accessor);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
    macros::CAM_ZOOM_OUT(fighter); 

    return 0.into();
}

// DOWN SMASH
unsafe extern "C" fn wario_attacklw4_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    DOWN_SMASH_AMOUNT[ENTRY_ID] = 0;

    return 0.into();
}

pub fn install() {
    Agent::new("wario")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_SPECIAL_HI, wario_specialhi_check_damage)
        .status(CheckDamage, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP, wario_specialhi_check_damage)

        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW4, wario_attacklw4_check_damage)
        
    .install();
}