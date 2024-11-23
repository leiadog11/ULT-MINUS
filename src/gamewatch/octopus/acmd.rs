use super::*;

// OCTOPUS NAIR
unsafe extern "C" fn gamewatch_octopus_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.2, 361, 90, 0, 60, 6.5, -10.0, 14.0, 0.0, Some(10.0), Some(14.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        // macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.2, 361, 90, 0, 60, 6.5, 0.0, 14.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        // macros::ATTACK(agent, 2, 0, Hash40::new("top"), 14.2, 361, 90, 0, 60, 6.5, 10.0, 14.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 3.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
}

//OCTOPUS NAIR EXPRESSION
unsafe extern "C" fn gamewatch_octopus_expression_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 7);
    }
}



pub fn install() {
    Agent::new("gamewatch_octopus")
        .game_acmd("game_attackairn", gamewatch_octopus_attackairn, Low)
        .expression_acmd("expression_attackairn", gamewatch_octopus_expression_attackairn, Low)
        .install();
}
