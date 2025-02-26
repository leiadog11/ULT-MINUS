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

//------------------WIN/LOSE--------------------

// WIN1
unsafe extern "C" fn falco_win1(agent: &mut L2CAgentBase) {
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    if macros::is_excute(agent) {
        if ENTRY_ID == 0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{x:-15.0,y:0.0,z:0.0});
        }
        else {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{x:15.0,y:0.0,z:0.0});
        }
    }
}

// WIN2 - WIN3
unsafe extern "C" fn falco_win(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("win_1"), 0.0, 1.0, false, 0.0, false, false);
    }
}

// LOSE
unsafe extern "C" fn falco_lose(agent: &mut L2CAgentBase) { 
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    if macros::is_excute(agent) { 
        if ENTRY_ID == 0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{x:-15.0,y:0.0,z:0.0});
        }
        else {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{x:15.0,y:0.0,z:0.0});
        }
    }
}

pub fn install() {
    Agent::new("falco")
    .game_acmd("game_appealsr", falco_appeals, Low)
    .sound_acmd("sound_appealsr", falco_sound_appeals, Low)

    .game_acmd("game_appealsl", falco_appeals, Low)
    .sound_acmd("sound_appealsl", falco_sound_appeals, Low)

    .game_acmd("game_win1", falco_win1, Low)

    .game_acmd("game_win2", falco_win, Low)
    .game_acmd("game_win3", falco_win, Low)

    .game_acmd("game_lose", falco_lose, Low)
    
    .install();
}