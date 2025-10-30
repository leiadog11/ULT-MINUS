use super::*;

//-------------------SPECIALS------------------

// DOWN B ROLL
unsafe extern "C" fn roy_speciallwroll(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// DOWN B ROLL EFFECT
unsafe extern "C" fn roy_effect_speciallwroll(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// DOWN B ROLL SOUND
unsafe extern "C" fn roy_sound_speciallwroll(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// DOWN B ROLL EXPRESSION
unsafe extern "C" fn roy_expression_speciallwroll(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// DOWN B DIVE
unsafe extern "C" fn roy_speciallwdive(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// DOWN B DIVE EFFECT
unsafe extern "C" fn roy_effect_speciallwdive(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// DOWN B DIVE SOUND
unsafe extern "C" fn roy_sound_speciallwdive(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

// DOWN B DIVE EXPRESSION
unsafe extern "C" fn roy_expression_speciallwdive(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 

    }
}

pub fn install() {
    Agent::new("roy")
        .game_acmd("game_speciallwroll", roy_speciallwroll, Low)
        .effect_acmd("effect_speciallwroll", roy_effect_speciallwroll, Low)
        .sound_acmd("sound_speciallwroll", roy_sound_speciallwroll, Low)
        .expression_acmd("expression_speciallwroll", roy_expression_speciallwroll, Low)

        .game_acmd("game_speciallwdive", roy_speciallwdive, Low)
        .effect_acmd("effect_speciallwdive", roy_effect_speciallwdive, Low)
        .sound_acmd("sound_speciallwdive", roy_sound_speciallwdive, Low)
        .expression_acmd("expression_speciallwdive", roy_expression_speciallwdive, Low)

        .install();
}