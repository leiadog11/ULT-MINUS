use super::*;

// FIREBALL ATTACK S4
unsafe extern "C" fn parachute_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 72, 55, 0, 45, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}

pub fn install() {
    Agent::new("gamewatch_parachute")
        .game_acmd("game_attacks4", parachute_attacks4, Low)

        .install();
}