use super::*;

//-------------------GLIDER SPECIALS------------------

// SPECIAL AIR HI START EFFECT
unsafe extern "C" fn parasail_effect_specialairhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1, false);
    }
}

// SPECIAL AIR HI GLIDE EFFECT
unsafe extern "C" fn parasail_effect_specialairhiglide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 3.6, 1.3, 0, 0, 0, 1.5, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 3.6, 1.3, 0, 0, 0, 1.5, false);
    }
}

// SPECIAL AIR HI UNEQUIP
unsafe extern "C" fn parasail_specialairhiunequip(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// SPECIAL AIR HI UNEQUIP EFFECT
unsafe extern "C" fn parasail_effect_specialairhiunequip(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("trans"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

// SPECIAL AIR HI EQUIP
unsafe extern "C" fn parasail_specialairhiequip(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// SPECIAL AIR HI EQUIP EFFECT
unsafe extern "C" fn parasail_effect_specialairhiequip(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) { 
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("trans"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("link_parasail")
        .effect_acmd("effect_specialairhistart", parasail_effect_specialairhistart, Low)

        .effect_acmd("effect_specialairhiglide", parasail_effect_specialairhiglide, Low)

        .game_acmd("game_specialairhiunequip", parasail_specialairhiunequip, Low)
        .effect_acmd("effect_specialairhiunequip", parasail_effect_specialairhiunequip, Low)

        .game_acmd("game_specialairhiequip", parasail_specialairhiequip, Low)
        .effect_acmd("effect_specialairhiequip", parasail_effect_specialairhiequip, Low)

        .install();
}