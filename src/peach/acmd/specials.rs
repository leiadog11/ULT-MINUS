use super::*;

//--------------------SPECIALS-----------------------

// NEUTRAL B
unsafe extern "C" fn peach_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if !macros::IS_EXIST_ARTICLE(agent, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO) { 
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, false, -1);
            ArticleModule::change_status(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, WEAPON_PEACH_KINOPIO_STATUS_KIND_APPEAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

// NEUTRAL B EFFECT
unsafe extern "C" fn peach_effect_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6, 7, 0, 0, 0, 1.6, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

// NEUTRAL B SOUND
unsafe extern "C" fn peach_sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_special_n01"));
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_special_n04"));
    }
}

// NEUTRAL B EXPRESSION
unsafe extern "C" fn peach_expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// AERIAL NEUTRAL B
unsafe extern "C" fn peach_specialairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, false, -1);
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, WEAPON_PEACH_KINOPIO_STATUS_KIND_AIR_SPRAY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// AERIAL NEUTRAL B EFFECT
unsafe extern "C" fn peach_effect_specialairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6, 7, 0, 0, 0, 1.6, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 13, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

// AERIAL NEUTRAL B SOUND
unsafe extern "C" fn peach_sound_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_special_n01"));
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_special_n04"));
    }
}

// AERIAL NEUTRAL B EXPRESSION
unsafe extern "C" fn peach_expression_specialairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// AERIAL DOWN B
unsafe extern "C" fn peach_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        if !macros::IS_EXIST_ARTICLE(agent, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO) { 
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, false, -1);
            ArticleModule::change_status(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, WEAPON_PEACH_KINOPIO_STATUS_KIND_FALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

// AERIAL DOWN B EFFECT
unsafe extern "C" fn peach_effect_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 6, 7, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 4, 8, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

// AERIAL DOWN B SOUND
unsafe extern "C" fn peach_sound_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_special_n01"));
    }
}

// AERIAL DOWN B EXPRESSION
unsafe extern "C" fn peach_expression_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("peach")
        .game_acmd("game_specialn", peach_specialn, Low)
        .effect_acmd("effect_specialn", peach_effect_specialn, Low)
        .sound_acmd("sound_specialn", peach_sound_specialn, Low)
        .expression_acmd("expression_specialn", peach_expression_specialn, Low)

        .game_acmd("game_specialairn", peach_specialairn, Low)
        .effect_acmd("effect_specialairn", peach_effect_specialairn, Low)
        .sound_acmd("sound_specialairn", peach_sound_specialairn, Low)
        .expression_acmd("expression_specialairn", peach_expression_specialairn, Low)

        .game_acmd("game_specialairlw", peach_specialairlw, Low)
        .effect_acmd("effect_specialairlw", peach_effect_specialairlw, Low)
        .sound_acmd("sound_specialairlw", peach_sound_specialairlw, Low)
        .expression_acmd("expression_specialairlw", peach_expression_specialairlw, Low)
        
        .install();
}