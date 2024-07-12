use super::*;

// FALL
unsafe extern "C" fn hydrant_fall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 270, 100, 0, 10, 5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(agent.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

pub fn install() {
    Agent::new("pacman_firehydrant")
        .game_acmd("game_fall", hydrant_fall, Low)
        .install();
}