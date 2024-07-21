use super::*;

//------------------------SPECIALS--------------------------

// NEUTRAL B
unsafe extern "C" fn palutena_specialn(agent: &mut L2CAgentBase) {
    
}

// NEUTRAL B EFFECT
unsafe extern "C" fn palutena_effect_specialn(agent: &mut L2CAgentBase) {

}

// NEUTRAL B SOUND
unsafe extern "C" fn palutena_sound_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_special_n01"));
    }
}

// NEUTRAL B EXPRESSION
unsafe extern "C" fn palutena_expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

// NEUTRAL B CHARGE EFFECT
unsafe extern "C" fn palutena_effect_specialncharge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    } 
}

// NEUTRAL B CHARGE SOUND
unsafe extern "C" fn palutena_sound_specialncharge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_palutena_attack100"));
    }
}

// NEUTRAL B CHARGE EXPRESSION
unsafe extern "C" fn palutena_expression_specialncharge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// NEUTRAL B SHOOT
unsafe extern "C" fn palutena_specialnshoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_RECT, 25, -10, 10, -15);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_BEAM, false, -1);
    }
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.5);
}

// NEUTRAL B SHOOT EFFECT
unsafe extern "C" fn palutena_effect_specialnshoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_final_gliderjump"), false, false);
    }
    frame(agent.lua_state_agent, 110.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_final_backlight"), false, false);
    }
}

// NEUTRAL B SHOOT SOUND
unsafe extern "C" fn palutena_sound_specialnshoot(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_palutena_final03"));
        macros::PLAY_STATUS(agent, Hash40::new("se_palutena_final01"));
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_palutena_final03"));
    }
    frame(agent.lua_state_agent, 68.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("se_palutena_final04"));
    }
}

// NEUTRAL B SHOOT EXPRESSION
unsafe extern "C" fn palutena_expression_specialnshoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionl"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_loopattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_specialn", palutena_specialn, Low)
        .effect_acmd("effect_specialn", palutena_effect_specialn, Low)
        .sound_acmd("sound_specialn", palutena_sound_specialn, Low)
        .expression_acmd("expression_specialn", palutena_expression_specialn, Low)

        .effect_acmd("effect_specialncharge", palutena_effect_specialncharge, Low)
        .sound_acmd("sound_specialncharge", palutena_sound_specialncharge, Low)
        .expression_acmd("expression_specialncharge", palutena_expression_specialncharge, Low)

        .game_acmd("game_specialnshoot", palutena_specialnshoot, Low)
        .effect_acmd("effect_specialnshoot", palutena_effect_specialnshoot, Low)
        .sound_acmd("sound_specialnshoot", palutena_sound_specialnshoot, Low)
        .expression_acmd("expression_specialnshoot", palutena_expression_specialnshoot, Low)

        .install();
}