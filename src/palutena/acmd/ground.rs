use super::*;

//------------------GROUND MOVES---------------------

// DASH ATTACK
unsafe extern "C" fn palutena_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("virtualshield"), *HIT_STATUS_INVINCIBLE);
    }
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 70, 76, 0, 71, 4.5, 0.0, 9.0, 13.0, Some(0.0), Some(11.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 60, 76, 0, 71, 3.5, 0.0, 10.0, 5.0, Some(0.0), Some(10.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 60, 56, 0, 61, 2.5, 0.0, 9.0, 10.0, Some(0.0), Some(12.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 30, 30, 30, 3.5, 0.0, 9.0, 9.7, Some(10.0), Some(12.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("virtualshield"), *HIT_STATUS_OFF);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// DASH ATTACK EFFECT
unsafe extern "C" fn palutena_effect_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_shield_flash"), Hash40::new("shield"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0, 11, 16, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("sys_flash"), -1);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) { 
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
    }
}

// DASH ATTACK SOUND
unsafe extern "C" fn palutena_sound_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_palutena_attackdash"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) { 
        macros::PLAY_SE(agent, Hash40::new("se_palutena_smash_h01"));
    }
    wait(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_palutena_step_right_s"));
    }
}

// DASH ATTACK EXPRESSION
unsafe extern "C" fn palutena_expression_attackdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 80, 300, 0.8, 0, 12, 24, 24, 50);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

// FORWARD TILT - FTILT
unsafe extern "C" fn palutena_attacks3(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.6);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_s4_charge"), false, -1.0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_s4s"), false, -1.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 40, 5.1, 0.0, 11.0, 6.0, Some(0.0), Some(9.9), Some(11.9), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 361, 102, 0, 40, 5.1, 0.0, 11.0, 6.0, Some(0.0), Some(8.5), Some(18.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 60, 0, 12.0, 0.0, 10.0, 8.0, Some(0.0), Some(10.0), Some(25.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 35, 0, 12.0, 0.0, 10.0, 8.0, Some(0.0), Some(10.0), Some(40.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// FORWARD TILT - FTILT EFFECT
unsafe extern "C" fn palutena_effect_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_smash_s_trace"), Hash40::new("top"), 0, 0, -15, 50, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 1, 21, 2.5, 0, -50, 0, 1, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6, -3, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6, -6, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

// FORWARD TILT - FTILT SOUND
unsafe extern "C" fn palutena_sound_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_smash_s"));
        macros::PLAY_SE(agent, Hash40::new("se_palutena_smash_s01"));
    }
}

// FORWARD TILT - FTILT EXPRESSION
unsafe extern "C" fn palutena_expression_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    execute(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::AREA_WIND_2ND_arg10(agent, 0, 2, 30, 300, 0.8, 25, 12, 50, 24, 80);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

// DOWN TILT
unsafe extern "C" fn palutena_attacklw3(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.6);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_lw4_charge"), false, -1.0);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_lw4"), false, -1.0);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 60, 103, 0, 52, 3.7, 0.0, 9.2, 7.3, Some(0.0), Some(9.2), Some(-7.3), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 28, 90, 0, 30, 6.0, 0.0, 4.0, 14.0, Some(0.0), Some(4.0), Some(-14.0), 0.85, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 55, 0, 9.0, 0.0, 4.0, 24.0, Some(0.0), Some(4.0), Some(-24.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// DOWN TILT EFFECT
unsafe extern "C" fn palutena_effect_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_smash_lw_trace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 3, 13.5, 0, 0, -90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 3, 8, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 3, -8, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
}

// DOWN TILT SOUND
unsafe extern "C" fn palutena_sound_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_smash_l"));
        macros::PLAY_SE(agent, Hash40::new("se_palutena_smash_l01"));
    }
}

// DOWN TILT EXPRESSION
unsafe extern "C" fn palutena_expression_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    execute(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        macros::AREA_WIND_2ND_arg10(agent, 0, 0.75, 110, 300, 0.8, 0, 15, 24, 30, 40);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 0.75, 70, 300, 0.8, 0, 12, 24, 24, 40);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_attackdash", palutena_attackdash, Low)
        .effect_acmd("effect_attackdash", palutena_effect_attackdash, Low)
        .sound_acmd("sound_attackdash", palutena_sound_attackdash, Low)
        .expression_acmd("expression_attackdash", palutena_expression_attackdash, Low)

        .game_acmd("game_attacks3", palutena_attacks3, Low)
        .effect_acmd("effect_attacks3", palutena_effect_attacks3, Low)
        .sound_acmd("sound_attacks3", palutena_sound_attacks3, Low)
        .expression_acmd("expression_attacks3", palutena_expression_attacks3, Low)

        .game_acmd("game_attacklw3", palutena_attacklw3, Low)
        .effect_acmd("effect_attacklw3", palutena_effect_attacklw3, Low)
        .sound_acmd("sound_attacklw3", palutena_sound_attacklw3, Low)
        .expression_acmd("expression_attacklw3", palutena_expression_attacklw3, Low)
        
        .install();
}