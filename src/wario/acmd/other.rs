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
    wait_loop_sync_mot(0);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 30, 144, 0, 120, 4.0, 0.0, 1.5, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// CROUCH EXPRESSION
unsafe extern "C" fn wario_expression_squatb(agent: &mut L2CAgentBase) {
    wait_loop_sync_mot(0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
}

pub fn install() {
    Agent::new("wario")
        .game_acmd("game_appeallwr", wario_appeallw, Low)
        .game_acmd("game_appeallwl", wario_appeallw, Low)

        //.game_acmd("game_squatb", wario_squatb, Low)
        //.expression_acmd("expression_squatb", wario_expression_squatb, Low)
        
        .install();
}