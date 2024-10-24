use super::*;

// T JOLT - REGULAR
unsafe extern "C" fn dengekidama_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 30, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 4.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}

pub fn install() {
    Agent::new("pichu_dengekidama")
        .game_acmd("game_regular", dengekidama_regular, Low)
        .install();
}