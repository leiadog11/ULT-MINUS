use super::*;

//---------------------SMASH ATTACKS------------------------

// UP SMASH
unsafe extern "C" fn palutena_attackhi4(agent: &mut L2CAgentBase) {
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    let pos_x = PostureModule::pos_x(agent.module_accessor);
    let pos_y = PostureModule::pos_y(agent.module_accessor);
    let lr = PostureModule::lr(agent.module_accessor);

    let mut LASER2_X = 0.0;
    let mut LASER2_Y = 0.0;

    if lr == -1.0 {
        LASER2_X = pos_x - BULLET_X_POS[ENTRY_ID];
    } 
    else {
        LASER2_X = BULLET_X_POS[ENTRY_ID] - pos_x;
    }

    LASER2_Y = BULLET_Y_POS[ENTRY_ID] - pos_y;

    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 88, 84, 0, 53, 5.5, 0.0, 121.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 88, 89, 0, 58, 3.8, 0.0, 144.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 88, 92, 0, 62, 3.8, 0.0, 165.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        if ANCHOR_PLANTED[ENTRY_ID] {
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 16.0, 88, 84, 0, 53, 5.5, 0.0, LASER2_Y, LASER2_X, Some(0.0), Some(LASER2_Y + 120.0), Some(LASER2_X), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 4, 0, Hash40::new("top"), 12.0, 88, 89, 0, 58, 3.8, 0.0, LASER2_Y, LASER2_X, Some(0.0), Some(LASER2_Y + 140.0), Some(LASER2_X), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 5, 0, Hash40::new("top"), 9.0, 88, 92, 0, 62, 3.8, 0.0, LASER2_Y, LASER2_X, Some(0.0), Some(LASER2_Y + 160.0), Some(LASER2_X), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 88, 84, 0, 53, 3.8, 0.0, 122.0, 9.7, Some(0.0), Some(2.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 88, 89, 0, 58, 3.8, 0.0, 144.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 88, 92, 0, 62, 3.8, 0.0, 165.0, 9.7, Some(0.0), Some(4.0), Some(9.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        if ANCHOR_PLANTED[ENTRY_ID] {
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 13.0, 88, 84, 0, 53, 3.8, 0.0, LASER2_Y, LASER2_X, Some(0.0), Some(LASER2_Y + 120.0), Some(LASER2_X), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 4, 0, Hash40::new("top"), 9.0, 88, 89, 0, 58, 3.8, 0.0, LASER2_Y, LASER2_X, Some(0.0), Some(LASER2_Y + 140.0), Some(LASER2_X), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
            macros::ATTACK(agent, 5, 0, Hash40::new("top"), 7.0, 88, 92, 0, 62, 3.8, 0.0, LASER2_Y, LASER2_X, Some(0.0), Some(LASER2_Y + 160.0), Some(LASER2_X), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        } 
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ANCHOR_PLANTED[ENTRY_ID] = false;
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// UP SMASH EFFECT
unsafe extern "C" fn palutena_effect_attackhi4(agent: &mut L2CAgentBase) {
    let ENTRY_ID = get_entry_id(agent.module_accessor);
    let pos_x = PostureModule::pos_x(agent.module_accessor);
    let pos_y = PostureModule::pos_y(agent.module_accessor);
    let lr = PostureModule::lr(agent.module_accessor);

    let mut LASER2_X = 0.0;
    let mut LASER2_Y = 0.0;

    if lr == -1.0 {
        LASER2_X = pos_x - BULLET_X_POS[ENTRY_ID];
    } 
    else {
        LASER2_X = BULLET_X_POS[ENTRY_ID] - pos_x;
    }

    LASER2_Y = BULLET_Y_POS[ENTRY_ID] - pos_y;

    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("stick"), 0, 8.5, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if ENTRY_ID == 0 { 
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 3, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0, 0);
            if ANCHOR_PLANTED[ENTRY_ID] { 
                macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, LASER2_Y - 5.0, LASER2_X, 0, 0, 0, 3, true);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0, 0);
            }
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    else if ENTRY_ID == 1 { 
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 3, true);
            if ANCHOR_PLANTED[ENTRY_ID] { 
                macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, LASER2_Y - 5.0, LASER2_X, 0, 0, 0, 3, true);
            }
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    else if ENTRY_ID == 2 { 
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 3, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0.255, 0);
            if ANCHOR_PLANTED[ENTRY_ID] { 
                macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, LASER2_Y - 5.0, LASER2_X, 0, 0, 0, 3, true);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0.255, 0);
            }
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    else if ENTRY_ID == 3 { 
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 3, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0.1275, 0);
            if ANCHOR_PLANTED[ENTRY_ID] { 
                macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, LASER2_Y - 5.0, LASER2_X, 0, 0, 0, 3, true);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0.1275, 0);
            }
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    else if ENTRY_ID == 4 { 
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 3, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0.1275, 0);
            if ANCHOR_PLANTED[ENTRY_ID] { 
                macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, LASER2_Y - 5.0, LASER2_X, 0, 0, 0, 3, true);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0.1275, 0);
            }
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    else if ENTRY_ID == 5 { 
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 3, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0, 0.204, 0.255);
            if ANCHOR_PLANTED[ENTRY_ID] { 
                macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, LASER2_Y - 5.0, LASER2_X, 0, 0, 0, 3, true);
                macros::LAST_EFFECT_SET_COLOR(agent, 0, 0.204, 0.255);
            }
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    else if ENTRY_ID == 6 { 
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 3, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0, 0.204);
            if ANCHOR_PLANTED[ENTRY_ID] { 
                macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, LASER2_Y - 5.0, LASER2_X, 0, 0, 0, 3, true);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.255, 0, 0.204);
            }
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    else if ENTRY_ID == 7 { 
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 3, true);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.204, 0, 0.255);
            if ANCHOR_PLANTED[ENTRY_ID] { 
                macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_pressure"), Hash40::new("top"), 0, LASER2_Y - 5.0, LASER2_X, 0, 0, 0, 3, true);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.204, 0, 0.255);
            }
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 10, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), 4, 21.5, 2, 0, -60, 0, 1, true, 0.7);
            macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

// FORWARD SMASH - FSMASH
unsafe extern "C" fn palutena_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_EXPLOSIVEFLAME, false, -1);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_EXPLOSIVEFLAME, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// FORWARD SMASH - FSMASH EFFECT
unsafe extern "C" fn palutena_effect_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 24, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light2"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("palutena_backlight"), Hash40::new("top"), -1, 21, 1, 0, 90, 0, 1, true, 0.7);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("palutena_wand_light2"), false, false);
    }
}

// FORWARD SMASH - FSMASH SOUND
unsafe extern "C" fn palutena_sound_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_palutena_special_s01"));
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_special_s"));
    }
}

// DOWN SMASH
unsafe extern "C" fn palutena_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, -1);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        CHARGE_MUL[get_entry_id(agent.module_accessor)] = 0.0;
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PALUTENA_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
}

// DOWN SMASH EFFECT
unsafe extern "C" fn palutena_effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 24, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

// DOWN SMASH SOUND
unsafe extern "C" fn palutena_sound_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_special_l02"));    
    }
}

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_attackhi4", palutena_attackhi4, Low)
        .effect_acmd("effect_attackhi4", palutena_effect_attackhi4, Low)

        .game_acmd("game_attacks4", palutena_attacks4, Low)
        .effect_acmd("effect_attacks4", palutena_effect_attacks4, Low)
        .sound_acmd("sound_attacks4", palutena_sound_attacks4, Low)

        .game_acmd("game_attacklw4", palutena_attacklw4, Low)
        .effect_acmd("effect_attacklw4", palutena_effect_attacklw4, Low)
        .sound_acmd("sound_attacklw4", palutena_sound_attacklw4, Low)
        
        .install();
}