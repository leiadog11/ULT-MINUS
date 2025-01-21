use super::*;

//-------------------GLIDER SPECIALS------------------

// SPECIAL AIR HI UNEQUIP
unsafe extern "C" fn parasail_specialairhiunequip(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// SPECIAL AIR HI EQUIP
unsafe extern "C" fn parasail_specialairhiequip(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

pub fn install() {
    Agent::new("link_parasail")
        .game_acmd("game_specialairhiunequip", parasail_specialairhiunequip, Low)

        .game_acmd("game_specialairhiequip", parasail_specialairhiequip, Low)

        .install();
}