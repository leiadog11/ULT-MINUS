use super::*;

//------------------TAUNTS---------------------

// SIDE TAUNT
unsafe extern "C" fn pacman_appeal_side(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 86.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 8.0, 361, 60, 0, 75, 9.0, 0.0, 5.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 87.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DOWN TAUNT
unsafe extern "C" fn pacman_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) { 
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_SLEEP_START, false.into());
    }
}

// WIN 1 EFFECT
unsafe extern "C" fn pacman_effect_win1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("cape"), true);
        macros::EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("rot"), 0, -4, 0, 0, 90, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("rot"), 0, -4, 0, 0, 90, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pacman_change_end"), Hash40::new("rot"), -1, 0, 0, 0, 0, 0, 1.3, false);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("cape"), true);
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 93.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

// WIN 2 EFFECT
unsafe extern "C" fn pacman_effect_win2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pacman_change_end"), Hash40::new("rot"), -1, 0, 0, 0, 0, 0, 1.3, false);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("cape"), true);
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 3, 0, 90, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}

// WIN 3 EFFECT
unsafe extern "C" fn pacman_effect_win3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("cape"), true);
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("pacman")
        .game_acmd("game_appealsr", pacman_appeal_side, Low)
        .game_acmd("game_appealsl", pacman_appeal_side, Low)

        .game_acmd("game_appeallwr", pacman_appeallw, Low)
        .game_acmd("game_appeallwl", pacman_appeallw, Low)

        .effect_acmd("effect_win1", pacman_effect_win1, Low)

        .effect_acmd("effect_win2", pacman_effect_win2, Low)

        .effect_acmd("effect_win3", pacman_effect_win3, Low)
        
        .install();
}