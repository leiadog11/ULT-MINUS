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

// CROUCH BACK
unsafe extern "C" fn wario_squatb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 30, 114, 0, 90, 4.0, 0.0, 4.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
}

pub fn install() {
    Agent::new("wario")
        .game_acmd("game_appeallwr", wario_appeallw, Low)
        .game_acmd("game_appeallwl", wario_appeallw, Low)

        .game_acmd("game_squatb", wario_squatb, Low)
        
        .install();
}