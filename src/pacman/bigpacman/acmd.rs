use super::*;

//----------------GHOST---------------

// START
unsafe extern "C" fn bigpacman_start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 92, 175, 0, 25, 8.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        for _ in 0..100 {
            macros::PLAY_SE(agent, Hash40::new("se_pacman_appeal_monster_ijike"));
            wait(agent.lua_state_agent, 30.0);
        }
    }
}

pub fn install() {
    Agent::new("pacman_bigpacman")
        .game_acmd("game_start", bigpacman_start, Low)
        .install();
}