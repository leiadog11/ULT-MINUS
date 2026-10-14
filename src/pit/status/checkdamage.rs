use super::*;

// UP B
unsafe extern "C" fn pit_specialhi_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_ikaros_wing_flare"), true, true);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("pit_fly_miracle_b"), true, true);
    macros::STOP_SE(fighter, Hash40::new("se_pit_flight_wings"));

    return 0.into();
}

pub fn install() {
    Agent::new("pit")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_SPECIAL_HI, pit_specialhi_check_damage)
        .status(CheckDamage, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, pit_specialhi_check_damage)
    
    .install();
}