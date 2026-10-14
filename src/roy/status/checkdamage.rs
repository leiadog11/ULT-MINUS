use super::*;

// TAUNT
unsafe extern "C" fn roy_taunt_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    remove_pyra(fighter.module_accessor);

    return 0.into();
}

pub fn install() {
    Agent::new("roy")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_APPEAL, roy_taunt_check_damage)
        
    .install();
}