use super::*;

//-----------------BOOST----------------

// BOOST EFFECT
unsafe extern "C" fn pump_effect_boost(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("mario_pump_shoot"), Hash40::new("mouth"), 0, 0, 0, 0, 90, 0, 0.8, true);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("mario_pump_shoot"), false, true);
    }
}

pub fn install() {
    Agent::new("mario_pump")
        .effect_acmd("effect_boost", pump_effect_boost, Low)

        .install();
}