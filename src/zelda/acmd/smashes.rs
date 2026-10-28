use super::*;

//-----------------SMASH ATTACKS----------------

// UP SMASH
unsafe extern "C" fn zelda_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 40, 0, 3.5, 0.0, 12.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 90, 73, 0, 55, 9.0, 0.0, 26.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 90, 70, 0, 55, 9.0, 0.0, 26.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP SMASH EFFECT
unsafe extern "C" fn zelda_effect_attackhi4(agent: &mut L2CAgentBase) {
    let lr = PostureModule::lr(agent.module_accessor);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_atk_flash_s"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 23.5, 0.5, 0, 0, 0, 0.7, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("zelda_atk_air_hi_bomb"), Hash40::new("top"), 0, 23.5, 0.5, 0, 0, 0, 0.7, true);
        macros::EFFECT(agent, Hash40::new("zelda_atk_air_hi_impact"), Hash40::new("top"), 0, 23.5, 0.5, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);

    }
}

// UP SMASH SOUND
unsafe extern "C" fn zelda_sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_zelda_rnd_attack"));
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_zelda_attackair_h01"));
    }
}

// UP SMASH EXPRESSION
unsafe extern "C" fn zelda_expression_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("zelda")
        .game_acmd("game_attackhi4", zelda_attackhi4, Low)
        .effect_acmd("effect_attackhi4", zelda_effect_attackhi4, Low)
        .sound_acmd("sound_attackhi4", zelda_sound_attackhi4, Low)
        .expression_acmd("expression_attackhi4", zelda_expression_attackhi4, Low)
    .install();
}