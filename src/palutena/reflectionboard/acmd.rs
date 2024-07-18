use super::*;

// SHOOT
unsafe extern "C" fn reflectionboard_shoot(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.4, 130, 60, 60, 60, 5.0, 0.0, 8.5, 0.0, Some(0.0), Some(-4.5), Some(0.0), 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 5, 0.0, 6, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 7.0, false);
    }
}

// BREAK
unsafe extern "C" fn reflectionboard_break(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) { 
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.2, 100, 0, 70, 50, 7.0, 0.0, 8.5, 0.0, Some(0.0), Some(-4.5), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 10, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }

    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("palutena_reflectionboard")
        .game_acmd("game_shoot", reflectionboard_shoot, Low)
        .game_acmd("game_break", reflectionboard_break, Low)

        .install();
}