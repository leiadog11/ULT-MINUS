use super::*;

//-------------------SPECIALS------------------

// SIDE B
unsafe extern "C" fn roy_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) { 
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_ROY_GENERATE_ARTICLE_ROYSWORD, false, -1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, FIGHTER_ROY_GENERATE_ARTICLE_ROYSWORD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 55, 100, 10, 60, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(12.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 55, 100, 10, 50, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(12.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 25.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 25.0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

// SIDE B EFFECT
unsafe extern "C" fn roy_effect_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("eflame_blazeend_throw"), Hash40::new("top"), 0, 12, 4, 50, -93, 0, 0.7, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
       // macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_sword_open"), true, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 9, 14, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND(agent, Hash40::new("eflame_catch_hand"), false, true);
    }
}

// SIDE B SOUND
unsafe extern "C" fn roy_sound_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_eflame_special_s01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("vc_eflame_special_s01_rand"));
    }
}

// SIDE B EXPRESSION
unsafe extern "C" fn roy_expression_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_attacks"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

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
        .game_acmd("game_specials", roy_specials, Low)
        .effect_acmd("effect_specials", roy_effect_specials, Low)
        .sound_acmd("sound_specials", roy_sound_specials, Low)
        .expression_acmd("expression_specials", roy_expression_specials, Low)

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