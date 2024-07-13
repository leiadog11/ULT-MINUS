use super::*;

//---------------------SMASH ATTACKS------------------------

// UP SMASH
unsafe extern "C" fn palutena_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 88, 84, 0, 53, 4.5, 0.0, 121.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 88, 89, 0, 58, 2.8, 0.0, 144.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 88, 92, 0, 62, 2.8, 0.0, 165.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 88, 84, 0, 53, 2.8, 0.0, 122.0, 9.7, Some(0.0), Some(2.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 88, 89, 0, 58, 2.8, 0.0, 144.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 88, 92, 0, 62, 2.8, 0.0, 165.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP SMASH EFFECT
unsafe extern "C" fn palutena_effect_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("stick"), 0, 8.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 3, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_attackhi4", palutena_attackhi4, Low)
        .effect_acmd("effect_attackhi4", palutena_effect_attackhi4, Low)
        
        .install();
}