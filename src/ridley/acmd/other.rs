use super::*;

//----------------------OTHER------------------------

// DASH
unsafe extern "C" fn ridley_run(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.4, 44, 62, 0, 55, 16.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

// HEAVY LANDING + LANDING LIGHT EXPRESSION
unsafe extern "C" fn ridley_expression_landing(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

// AERIAL LANDING EXPRESSION
unsafe extern "C" fn ridley_expression_landingair(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// FORWARD AIR LANDING
unsafe extern "C" fn ridley_landingairf(agent: &mut L2CAgentBase) {
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 3.5, 40, 50, 0, 60, 9.5, -5.0, 0.0, 0.0, Some(5.0), None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
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
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
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

//----------------------WIN/LOSE------------------------

// LOSE
unsafe extern "C" fn ridley_lose(agent: &mut L2CAgentBase) { 
    macros::FT_MOTION_RATE(agent, 1.9);
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) { 
        if ENTRY_ID == 0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{x:-15.0,y:0.0,z:0.0});
        }
        else {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{x:15.0,y:0.0,z:0.0});
        }
    }
}

pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_run", ridley_run, Low)

        .game_acmd("game_landingairf", ridley_landingairf, Low)

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

        .game_acmd("game_lose", ridley_lose, Low)
        
        .install();
}