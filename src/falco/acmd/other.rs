use super::*;

//------------------TAUNTS--------------------

//SIDE TAUNT FACING RIGHT
unsafe extern "C" fn falco_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 1.0);
    }
}

//SIDE TAUNT SOUND EFFECT
unsafe extern "C" fn falco_sound_appeals(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_03"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_falco_appeal02"));
    }
}

pub fn install() {
    Agent::new("falco")
    .game_acmd("game_appealsr", falco_appeals, Low)
    .sound_acmd("sound_appealsr", falco_sound_appeals, Low)

    .game_acmd("game_appealsl", falco_appeals, Low)
    .sound_acmd("sound_appealsl", falco_sound_appeals, Low)
    
    .install();
}