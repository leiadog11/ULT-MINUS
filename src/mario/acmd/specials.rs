use super::*;

//-------------------SPECIALS------------------

// NEUTRAL B EFFECT
unsafe extern "C" fn mario_effect_specialn(agent: &mut L2CAgentBase) {
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    let rand = smash::app::sv_math::rand(hash40("agent"), 4) as u64;
    if rand < 2 { 
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            ICEBALL[ENTRY_ID] = true;
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 0.5, true);
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
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            ICEBALL[ENTRY_ID] = false; 
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_freezer"), false, false);
        }
    }
    else {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, 45, 0, 1, true);
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
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("mario_fb_shoot"), false, false);
        }
    }
}

// NEUTRAL B SOUND
unsafe extern "C" fn mario_sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        
    }
    if ICEBALL[get_entry_id(agent.module_accessor)] {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_frieze_m"));
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
        .effect_acmd("effect_specialn", mario_effect_specialn, Low)
        .sound_acmd("sound_specialn", mario_sound_specialn, Low)

        .install();
}