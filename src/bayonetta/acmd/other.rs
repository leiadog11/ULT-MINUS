use super::*;

// AIR JUMP EFFECT
unsafe extern "C" fn bayonetta_effect_jumpaerial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

// AIR JUMP SOUND
unsafe extern "C" fn bayonetta_sound_jumpaerial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_bayonetta_jump01"));
    }
}

pub fn install() {
    Agent::new("bayonetta")
        .effect_acmd("effect_jumpaerialfront", bayonetta_effect_jumpaerial, Low)
        .effect_acmd("effect_jumpaerialback", bayonetta_effect_jumpaerial, Low)
        .sound_acmd("sound_jumpaerialfront", bayonetta_sound_jumpaerial, Low)
        .sound_acmd("sound_jumpaerialback", bayonetta_sound_jumpaerial, Low)
    .install();
}