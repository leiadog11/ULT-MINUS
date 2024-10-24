use super::*;

// BIKE HITBOX
unsafe extern "C" fn wario_wariobike_specialsdrive(agent: &mut L2CAgentBase) {
    WorkModule::set_int(agent.module_accessor, 10, WEAPON_WARIO_WARIOBIKE_STATUS_WORK_INT_BIKE_JUMP);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 9.0, 361, 50, 0, 50, 5.0, 0.0, 2.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 9.0, 361, 50, 0, 50, 3.0, 0.0, 2.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        JostleModule::set_status(agent.module_accessor, false);
    }
}

pub fn install() {
    Agent::new("wario_wariobike")
        .game_acmd("game_specialsdrive", wario_wariobike_specialsdrive, Low)
        
        .install();
}