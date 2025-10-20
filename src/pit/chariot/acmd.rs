use super::*;

// SHIELD
unsafe extern "C" fn chariot_shield(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 1, *FIGHTER_PIT_SHIELD_GROUP_KIND_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        macros::ATTACK(agent, 0, 0, Hash40::new("virtualguardianf"), 0.0, 50, 100, 50, 0, 7.0, 0.0, 8.0, 9.0, Some(0.0), Some(1.0), None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("virtualguardianf"), 0.0, 50, 100, 50, 0, 7.0, 0.0, 8.0, -9.0, Some(0.0), Some(1.0), None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

// SHIELD EFFECT
unsafe extern "C" fn chariot_effect_shield(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    for _ in 0..60 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("pit_guardian_shield"), Hash40::new("virtualguardianf"), 2, 8, 9, 0, 300, 0, 1.5, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("pit_guardian_shield"), Hash40::new("virtualguardianf"), 2, 8, -9, 0, 60, 0, 1.5, true);
        }
        for _ in 0..10 {
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
        wait(agent.lua_state_agent, 1.0);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_guardian_shield"), true, true);
    }
}

// SHIELD SOUND
unsafe extern "C" fn chariot_sound_shield(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pit_special_l01"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pit_rnd_special_l"));
    }
}

// BREAK EFFECT
unsafe extern "C" fn chariot_effect_break(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_guardian_shield"), true, true);
        macros::EFFECT(agent, Hash40::new("pit_guardian_break"), Hash40::new("virtualguardianf"), 2, 3, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("pit_guardian_break"), Hash40::new("virtualguardianf"), 2, 3, -6, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("virtualguardianf"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

// BREAK SOUND
unsafe extern "C" fn chariot_sound_break(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pit_special_l02"));
    }
}

// END EFFECT
unsafe extern "C" fn chariot_effect_end(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("pit_guardian_shield_end"), Hash40::new("virtualguardianf"), 2, 3, 4, 0, 300, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("pit_guardian_shield_end"), Hash40::new("virtualguardianf"), 2, 3, -4, 0, 60, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pit_guardian_shield"), false, false);
    }
}

pub fn install() {
    Agent::new("pit_chariot")

    .game_acmd("game_shield", chariot_shield, Low)
    .effect_acmd("effect_shield", chariot_effect_shield, Low)
    .sound_acmd("sound_shield", chariot_sound_shield, Low)

    .effect_acmd("effect_break", chariot_effect_break, Low)
    .sound_acmd("sound_break", chariot_sound_break, Low)

    .effect_acmd("effect_end", chariot_effect_end, Low)

    .install();
}