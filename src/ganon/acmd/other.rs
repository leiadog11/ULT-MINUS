use super::*;

//--------------TAUNTS-----------------

// DOWN TAUNT
unsafe extern "C" fn ganon_appeallw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        SWORD[get_entry_id(agent.module_accessor)] = true;
    }

}

// UP TAUNT
unsafe extern "C" fn ganon_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) { 
        for _ in 0..15 {
            macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 0.5, 361, 30, 0, 20, 7.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_KICK);
            wait(agent.lua_state_agent, 2.0);
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) { 
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 7.0, 55, 60, 0, 60, 9.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_appeallwl", ganon_appeallw, Low)
        .game_acmd("game_appeallwr", ganon_appeallw, Low)

        .game_acmd("game_appealhir", ganon_appealhi, Low)
        .game_acmd("game_appealhil", ganon_appealhi, Low)
        
        .install();
}