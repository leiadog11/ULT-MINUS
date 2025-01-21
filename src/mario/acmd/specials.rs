use super::*;

//-------------------SPECIALS------------------

// NEUTRAL B EFFECT
unsafe extern "C" fn mario_effect_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ICEBALL) { 
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1, true);
        }
    }
    else {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 0, 0, 0.353);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ICEBALL) { 
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_freezer"), false, false);
        }
    }
    else {
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, false);
        }
    }
}

// NEUTRAL B SOUND
unsafe extern "C" fn mario_sound_specialn(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_ICEBALL) {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_frieze_s"));
        }
    }
    else {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_mario_special_n01"));
        }
    }
}

pub fn install() {
    Agent::new("mario")
        .install();
}