use super::*;

//----------------------OTHER------------------------

// DASH
unsafe extern "C" fn ridley_run(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.4, 44, 62, 0, 55, 16.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

// HEAVY LANDING + LANDING LIGHT EXPRESSION
unsafe extern "C" fn ridley_expression_landing(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

// AERIAL LANDING EXPRESSION
unsafe extern "C" fn ridley_expression_landingair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// LANDING SOUND
unsafe extern "C" fn ridley_sound_landing(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_landing03"));
    }
}

// UP TAUNT
unsafe extern "C" fn ridley_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.1, 44, 62, 0, 55, 25.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP TAUNT EFFECT
unsafe extern "C" fn ridley_effect_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_clatter"), Hash40::new("top"), -2, 15, 0, 0, 0, 0, 3.0, true);
        }
        wait(agent.lua_state_agent, 13.0);
    }
}

// UP TAUNT EXPRESSION
unsafe extern "C" fn ridley_expression_appealhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_aerial"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_aerial"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_aerial"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}


pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_run", ridley_run, Low)

        .sound_acmd("sound_landingheavy", ridley_sound_landing, Low)
        .sound_acmd("sound_landinglight", ridley_sound_landing, Low)
        .expression_acmd("expression_landingheavy", ridley_expression_landing, Low)
        .expression_acmd("expression_landinglight", ridley_expression_landing, Low)
        
        .sound_acmd("sound_landingairb", ridley_expression_landing, Low)
        .sound_acmd("sound_landingairf", ridley_expression_landing, Low)
        .sound_acmd("sound_landingairhi", ridley_expression_landing, Low)
        .sound_acmd("sound_landingairlw", ridley_expression_landing, Low)
        .expression_acmd("expression_landingairb", ridley_expression_landingair, Low)
        .expression_acmd("expression_landingairf", ridley_expression_landingair, Low)
        .expression_acmd("expression_landingairhi", ridley_expression_landingair, Low)
        .expression_acmd("expression_landingairlw", ridley_expression_landingair, Low)

        .game_acmd("game_appealhir", ridley_appealhi, Low)
        .game_acmd("game_appealhil", ridley_appealhi, Low)
        .effect_acmd("effect_appealhir", ridley_effect_appealhi, Low)
        .effect_acmd("effect_appealhil", ridley_effect_appealhi, Low)
        .expression_acmd("expression_appealhir", ridley_expression_appealhi, Low)
        .expression_acmd("expression_appealhil", ridley_expression_appealhi, Low)
        
        .install();
}