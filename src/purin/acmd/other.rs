use super::*;

//--------------TAUNTS-----------------

// DOWN TAUNT
unsafe extern "C" fn purin_appeallw(agent: &mut L2CAgentBase) {
    AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 78.0);
    if macros::is_excute(agent) {
        if true {
            macros::FT_MOTION_RATE(agent, 1000.0);
        }
    }
}

//---------------SHIELD----------------

//SHIELD BREAK
unsafe extern "C" fn purin_shieldbreakfly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 90, 1000, 0, 300, 8.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
}

pub fn install() {
    Agent::new("purin")
        .game_acmd("game_appeallwr", purin_appeallw, Low)
        .game_acmd("game_appeallwl", purin_appeallw, Low)

        .game_acmd("game_shieldbreakfly", purin_shieldbreakfly, Low)
        
        .install();
}