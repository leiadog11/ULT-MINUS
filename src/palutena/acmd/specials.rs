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
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) { 
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_BEAM, false, -1);
    }
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.7);
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) { 
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_BEAM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_BLACKHOLE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// NEUTRAL B SHOOT EFFECT
unsafe extern "C" fn palutena_effect_specialnshoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("palutena_final_ripple"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
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

// SIDE B
unsafe extern "C" fn palutena_specials(agent: &mut L2CAgentBase) {
    
}

// SIDE B EFFECT
unsafe extern "C" fn palutena_effect_specials(agent: &mut L2CAgentBase) {

}

// SIDE B SOUND
unsafe extern "C" fn palutena_sound_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_special_n01"));
    }
}

// SIDE B EXPRESSION
unsafe extern "C" fn palutena_expression_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

// SIDE B CHARGE EFFECT
unsafe extern "C" fn palutena_effect_specialscharge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let dumb = Vector3f{x:0.0,y:9.0,z:0.0};
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -0.2, 22, -1, 10, 90, 0, 1, true);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_trace"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        let effect = EffectModule::req_follow(agent.module_accessor, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), &dumb, &dumb, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(agent.module_accessor, effect.try_into().unwrap(), 0.5, 0.0, 0.5);
        //macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    } 
}

// SIDE B CHARGE SOUND
unsafe extern "C" fn palutena_sound_specialscharge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_palutena_attack100"));
    }
}

// SIDE B CHARGE EXPRESSION
unsafe extern "C" fn palutena_expression_specialscharge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// SIDE B SHOOT
unsafe extern "C" fn palutena_specialsshoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_RECT, 25, -10, 10, -15);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) { 
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_BLACKHOLE, false, -1);
    }
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.7);
    frame(agent.lua_state_agent, 65.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) { 
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_BLACKHOLE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// SIDE B SHOOT EFFECT
unsafe extern "C" fn palutena_effect_specialsshoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("palutena_final_ripple"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_final_gliderjump"), false, false);
    }
    frame(agent.lua_state_agent, 110.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_final_backlight"), false, false);
    }
}

// SIDE B SHOOT SOUND
unsafe extern "C" fn palutena_sound_specialsshoot(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_palutena_final02"));
        macros::PLAY_STATUS(agent, Hash40::new("se_palutena_final02"));
    }
}

// SIDE B SHOOT EXPRESSION
unsafe extern "C" fn palutena_expression_specialsshoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionl"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// DOWN B
unsafe extern "C" fn palutena_speciallw(agent: &mut L2CAgentBase) {
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::FT_MOTION_RATE(agent, 0.5);
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) { 
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, false, -1);
    }
}

// DOWN B EFFECT
unsafe extern "C" fn palutena_effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 5, 23, 0, 0, -90, 0, 1, true, 0.7);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_appeal_feather"), Hash40::new("top"), 0, 18, 0, 0, 0, 0, 1, true);
    }
}

// DOWN B SOUND
unsafe extern "C" fn palutena_sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_palutena_attack06"));
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_appeal_h01"));
    }
}

// DOWN B EXPRESSION
unsafe extern "C" fn palutena_expression_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// DOWN B TP
unsafe extern "C" fn palutena_speciallwtp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// DOWN B TP EFFECT
unsafe extern "C" fn palutena_effect_speciallwtp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_teleport_end"), Hash40::new("rot"), 0, 4, 0, 0, 0, 0, 1, false);
        macros::FLASH(agent, 1, 1, 1, 1);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 0, 21.5, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 4);
        macros::FLASH(agent, 1, 1, 1, 0.3);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 0.25, 1, 1, 0.2);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_backlight"), false, true);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FLASH(agent, 1, 1, 1, 0.15);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_teleport_feather"), Hash40::new("rot"), 0, 4, 0, 0, 0, 0, 1, true);
    }
}

// DOWN B TP SOUND
unsafe extern "C" fn palutena_sound_speciallwtp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_special_h02"));
    }
}

// DOWN B TP EXPRESSION
unsafe extern "C" fn palutena_expression_speciallwtp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
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

        .game_acmd("game_specials", palutena_specials, Low)
        .effect_acmd("effect_specials", palutena_effect_specials, Low)
        .sound_acmd("sound_specials", palutena_sound_specials, Low)
        .expression_acmd("expression_specials", palutena_expression_specials, Low)

        .effect_acmd("effect_specialscharge", palutena_effect_specialscharge, Low)
        .sound_acmd("sound_specialscharge", palutena_sound_specialscharge, Low)
        .expression_acmd("expression_specialscharge", palutena_expression_specialscharge, Low)

        .game_acmd("game_specialsshoot", palutena_specialsshoot, Low)
        .effect_acmd("effect_specialsshoot", palutena_effect_specialsshoot, Low)
        .sound_acmd("sound_specialsshoot", palutena_sound_specialsshoot, Low)
        .expression_acmd("expression_specialsshoot", palutena_expression_specialsshoot, Low)

        .game_acmd("game_speciallw", palutena_speciallw, Low)
        .effect_acmd("effect_speciallw", palutena_effect_speciallw, Low)
        .sound_acmd("sound_speciallw", palutena_sound_speciallw, Low)
        .expression_acmd("expression_speciallw", palutena_expression_speciallw, Low)

        .game_acmd("game_speciallwtp", palutena_speciallwtp, Low)
        .effect_acmd("effect_speciallwtp", palutena_effect_speciallwtp, Low)
        .sound_acmd("sound_speciallwtp", palutena_sound_speciallwtp, Low)
        .expression_acmd("expression_speciallwtp", palutena_expression_speciallwtp, Low)

        .install();
}