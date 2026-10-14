use super::*;

// CAPE DAMAGE CHECK
unsafe extern "C" fn pacman_cape_check_damage(fighter: &mut L2CFighterCommon, param2: &L2CValue) -> L2CValue {
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("cape"), true);

    return 0.into();
}

pub fn install() {
    Agent::new("pacman")
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_DASH, pacman_cape_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_LW3, pacman_cape_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_ATTACK_AIR, pacman_cape_check_damage)
        .status(CheckDamage, *FIGHTER_STATUS_KIND_SPECIAL_HI, pacman_cape_check_damage)
        .status(CheckDamage, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP, pacman_cape_check_damage)
        .status(CheckDamage, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_END, pacman_cape_check_damage)
        .status(CheckDamage, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_MOVE, pacman_cape_check_damage)
        .status(CheckDamage, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH, pacman_cape_check_damage)
        .status(CheckDamage, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN, pacman_cape_check_damage)
        
    .install();
}