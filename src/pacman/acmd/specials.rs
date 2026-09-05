use super::*;

//--------------------SPECIALS-----------------------

// SIDE B DASH
unsafe extern "C" fn pacman_specialsdash(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_S_WORK_FLAG_EAT_POWER_ESA) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 50, 108, 0, 50, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            blastzone_check(agent, 80.0);
        }
        else {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 361, 66, 0, 40, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            }
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        blastzone_check(agent, 80.0);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        blastzone_check(agent, 80.0);
    }
}

// SPECIAL S RETURN
unsafe extern "C" fn pacman_specialsreturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        GroundModule::set_collidable(agent.module_accessor, true);
    }
}

// SPECIAL FALL LANDING
unsafe extern "C" fn pacman_effect_landingfallspecial(agent: &mut L2CAgentBase) { 
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("cape"), true);
    }
}

pub fn install() {
    Agent::new("pacman")
        .game_acmd("game_specialsdash", pacman_specialsdash, Low)

        .game_acmd("game_specialsreturn", pacman_specialsreturn, Low)
        .game_acmd("game_specialairsreturn", pacman_specialsreturn, Low)

        .effect_acmd("effect_landingfallspecial", pacman_effect_landingfallspecial, Low)
        
        .install();
}