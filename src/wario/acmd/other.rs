use super::*;

//-------------TAUNTS--------------------

// DOWN TAUNT
unsafe extern "C" fn wario_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_WARIOBIKE), 0, 0, false, false);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        ItemModule::drop_item(agent.module_accessor, 90.0, 0.0, 0);
    }
}

pub fn install() {
    Agent::new("wario")
        .game_acmd("game_appeallwr", wario_appeallw, Low)
        .game_acmd("game_appeallwl", wario_appeallw, Low)
        
        .install();
}