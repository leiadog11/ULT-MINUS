use super::*;

//------------------ ACMD --------------------

unsafe extern "C" fn doyle_specials2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        
    }
}

pub fn install() {
    Agent::new("jack_doyle")
        .game_acmd("game_specials2", doyle_specials2, Low)

        .install();
}