use super::*;

//----------------------OTHER------------------------

// DASH
unsafe extern "C" fn ridley_run(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.4, 44, 62, 0, 55, 16.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
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


pub fn install() {
    Agent::new("ridley")
        .game_acmd("game_run", ridley_run, Low)

        .expression_acmd("expression_landingheavy", ridley_expression_landing, Low)
        .expression_acmd("expression_landinglight", ridley_expression_landing, Low)

        .expression_acmd("expression_landingairb", ridley_expression_landingair, Low)
        .expression_acmd("expression_landingairf", ridley_expression_landingair, Low)
        .expression_acmd("expression_landingairhi", ridley_expression_landingair, Low)
        .expression_acmd("expression_landingairlw", ridley_expression_landingair, Low)
        
        .install();
}