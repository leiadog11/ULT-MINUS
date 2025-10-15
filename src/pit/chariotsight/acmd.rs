use super::*;

// SHIELD
unsafe extern "C" fn chariotsight_shield(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 1, *FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        macros::ATTACK(agent, 0, 0, Hash40::new("virtualguardianf)", 0.0, 50, 100, 50, 0, 2.7, 0.0, -2.0, 1.0, Some(0.0), Some(3.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("virtualguardianb)", 0.0, 50, 100, 50, 0, 2.7, 0.0, -2.0, -1.0, Some(0.0), Some(3.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

// SHIELD EFFECT
unsafe extern "C" fn chariotsight_effect_shield(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pit_guardian_shield"), Hash40::new("virtualguardianf)", 2, 3, 4, 0, 300, 0, 1.5, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pit_guardian_shield"), Hash40::new("virtualguardianb)", 2, 3, -4, 0, 60, 0, 1.5, true);
    }
    for _ in 0..100 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.897, 0.855, 0.313, 0.1);
            macros::BURN_COLOR(agent, 16, 16, 10, 0.1);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 0.96, 0.55, 0);
            macros::BURN_COLOR_FRAME(agent, 2, 16, 16, 16, 0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 1, 1, 0.1);
            macros::BURN_COLOR_FRAME(agent, 1, 16, 16, 10, 0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::BURN_COLOR_FRAME(agent, 1, 16, 16, 16, 0.1);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}

// SHIELD SOUND
unsafe extern "C" fn chariotsight_sound_shield(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pit_special_l01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pit_rnd_special_l"));
    }
}

// SHIELD EXPRESSION
unsafe extern "C" fn chariotsight_expression_shield(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// BREAK EFFECT
unsafe extern "C" fn chariotsight_effect_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_guardian_shield"), true, true);
        macros::EFFECT(agent, Hash40::new("pit_guardian_break"), Hash40::new("virtualguardianf)", 2, 3, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("pit_guardian_break"), Hash40::new("virtualguardianb)", 2, 3, -6, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

// BREAK SOUND
unsafe extern "C" fn chariotsight_sound_break(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pit_special_l02"));
    }
}

// BREAK EXPRESSION
unsafe extern "C" fn chariotsight_expression_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_shieldbreak"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

pub fn install() {
    Agent::new("pit_chariotsight")

    .game_acmd("game_shield", chariotsight_shield, Low)
    .effect_acmd("effect_shield", chariotsight_effect_shield, Low)
    .sound_acmd("sound_shield", chariotsight_sound_shield, Low)
    .expression_acmd("expression_shield", chariotsight_expression_shield, Low)

    .effect_acmd("effect_break", chariotsight_effect_break, Low)
    .sound_acmd("sound_break", chariotsight_sound_break, Low)
    .expression_acmd("expression_break", chariotsight_expression_break, Low)

    .install();
}