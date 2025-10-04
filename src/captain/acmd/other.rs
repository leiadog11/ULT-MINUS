use super::*;

// ----------- TAUNTS -----------

// UP TAUNT
unsafe extern "C" fn captain_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    for _ in 0..17 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 110, 40, 0, 25, 11.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

pub fn install() {
    Agent::new("captain")
        .game_acmd("game_appealhir", captain_appealhi, Low)
        .game_acmd("game_appealhil", captain_appealhi, Low)

        .install();
}