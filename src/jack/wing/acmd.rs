use super::*;

//AERIAL UP SPECIAL
unsafe extern "C" fn jack_specialairhistart(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.7);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
}

//GROUNDED UP SPECIAL
unsafe extern "C" fn jack_specialhistart(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.7);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
}

pub fn install() {
    Agent::new("jack")
        .game_acmd("game_specialairhistart", jack_specialairhistart, Low)

        .game_acmd("game_specialhistart", jack_specialhistart, Low)

        .install();
}