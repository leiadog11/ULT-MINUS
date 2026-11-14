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

// AIR DODGE
unsafe extern "C" fn bayonetta_escapeair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

// DIRECTIONAL AIR DODGE
unsafe extern "C" fn bayonetta_escapeairslide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

// ROLL BACK
unsafe extern "C" fn bayonetta_escapeb(agent: &mut L2CAgentBase) {

}

// ROLL FORWARD
unsafe extern "C" fn bayonetta_escapef(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
}

// SPOT DODGE
unsafe extern "C" fn bayonetta_escapen(agent: &mut L2CAgentBase) {

}

pub fn install() {
    Agent::new("bayonetta")
        .effect_acmd("effect_jumpaerialfront", bayonetta_effect_jumpaerial, Low)
        .effect_acmd("effect_jumpaerialback", bayonetta_effect_jumpaerial, Low)
        .sound_acmd("sound_jumpaerialfront", bayonetta_sound_jumpaerial, Low)
        .sound_acmd("sound_jumpaerialback", bayonetta_sound_jumpaerial, Low)

        .game_acmd("game_escapeair", bayonetta_escapeair, Low)
        .game_acmd("game_escapeairslide", bayonetta_escapeairslide, Low)
        .game_acmd("game_escapeb", bayonetta_escapeb, Low)
        .game_acmd("game_escapef", bayonetta_escapef, Low)
        .game_acmd("game_escapen", bayonetta_escapen, Low)
        
    .install();
}