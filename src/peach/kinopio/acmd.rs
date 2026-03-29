use super::*;

//--------------------KINOPIO-----------------------

// APPEAR
unsafe extern "C" fn kinopio_appear(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// APPEAR EFFECT
unsafe extern "C" fn kinopio_effect_appear(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// APPEAR SOUND
unsafe extern "C" fn kinopio_sound_appear(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// WAIT
unsafe extern "C" fn kinopio_wait(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// WAIT EFFECT
unsafe extern "C" fn kinopio_effect_wait(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// WAIT SOUND
unsafe extern "C" fn kinopio_sound_wait(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// PUNCH
unsafe extern "C" fn kinopio_punch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// PUNCH EFFECT
unsafe extern "C" fn kinopio_effect_punch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// PUNCH SOUND
unsafe extern "C" fn kinopio_sound_punch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// SPRAY
unsafe extern "C" fn kinopio_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// SPRAY EFFECT
unsafe extern "C" fn kinopio_effect_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// SPRAY SOUND
unsafe extern "C" fn kinopio_sound_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// AIR SPRAY
unsafe extern "C" fn kinopio_air_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// AIR SPRAY EFFECT
unsafe extern "C" fn kinopio_effect_air_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// AIR SPRAY SOUND
unsafe extern "C" fn kinopio_sound_air_spray(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// FALL
unsafe extern "C" fn kinopio_fall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// FALL EFFECT
unsafe extern "C" fn kinopio_effect_fall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// FALL SOUND
unsafe extern "C" fn kinopio_sound_fall(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// LAND
unsafe extern "C" fn kinopio_land(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// LAND EFFECT
unsafe extern "C" fn kinopio_effect_land(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

// LAND SOUND
unsafe extern "C" fn kinopio_sound_land(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {

    }
}

pub fn install() {
    Agent::new("peach_kinopio")
        .game_acmd("game_appear", kinopio_appear, Low)
        .effect_acmd("effect_appear", kinopio_effect_appear, Low)
        .sound_acmd("sound_appear", kinopio_sound_appear, Low)

        .game_acmd("game_wait", kinopio_wait, Low)
        .effect_acmd("effect_wait", kinopio_effect_wait, Low)
        .sound_acmd("sound_wait", kinopio_sound_wait, Low)

        .game_acmd("game_punch", kinopio_punch, Low)
        .effect_acmd("effect_punch", kinopio_effect_punch, Low)
        .sound_acmd("sound_punch", kinopio_sound_punch, Low)

        .game_acmd("game_spray", kinopio_spray, Low)
        .effect_acmd("effect_spray", kinopio_effect_spray, Low)
        .sound_acmd("sound_spray", kinopio_sound_spray, Low)

        .game_acmd("game_air_spray", kinopio_air_spray, Low)
        .effect_acmd("effect_air_spray", kinopio_effect_air_spray, Low)
        .sound_acmd("sound_air_spray", kinopio_sound_air_spray, Low)

        .game_acmd("game_fall", kinopio_fall, Low)
        .effect_acmd("effect_fall", kinopio_effect_fall, Low)
        .sound_acmd("sound_fall", kinopio_sound_fall, Low)

        .game_acmd("game_land", kinopio_land, Low)
        .effect_acmd("effect_land", kinopio_effect_land, Low)
        .sound_acmd("sound_land", kinopio_sound_land, Low)
        
        .install();
}