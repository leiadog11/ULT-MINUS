use super::*;

//-------------------SPECIALS------------------

// NEUTRAL B
unsafe extern "C" fn link_specialnstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
    }
}

// UP B
unsafe extern "C" fn link_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        if WorkModule::get_float(agent.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME) >= 59.0 { 
            if URBOSAS_FURY { 
                macros::PLAY_SE(agent, Hash40::new("se_link_spirit_activate"));
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 2.0, 10.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -2.0, 10.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 4.0, 12.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -4.0, 12.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 6.0, 14.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -6.0, 14.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("trans"), 8.0, 16.0, 0.0, 0, 0, 0, 0.8, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_aura"), Hash40::new("trans"), -8.0, 16.0, 0.0, 0, 0, 0, 0.8, false);
            }
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 14.0, 45, 88, 0, 60, 4.0, -2.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 14.0, 45, 88, 0, 60, 3.5, 2.5, 0.0, 1.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 11.2, 45, 88, 0, 60, 2.8, 8.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 12.0, 43, 88, 0, 52, 4.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 12.0, 43, 88, 0, 52, 3.5, 2.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 9.6, 43, 88, 0, 52, 2.8, 8.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) { 
        // URBOSAS FURY
        if WorkModule::get_float(agent.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME) >= 59.0 {  
            if URBOSAS_FURY { 
                URBOSAS_FURY = false;
                macros::PLAY_SE(agent, Hash40::new("se_link_spirit_lightning"));
                macros::ATTACK(agent, 3, 0, Hash40::new("hip"), 12.0, 43, 88, 0, 52, 35.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("hip"), 0.0, 0.0, 0.0, 0, 0, 0, 2.0, true);
                macros::LAST_EFFECT_SET_COLOR(agent, 0.200, 0.255, 0.050);
            }
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 9.0, 43, 84, 0, 48, 4.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 9.0, 43, 84, 0, 48, 3.5, 2.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 7.2, 43, 84, 0, 48, 2.8, 8.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword2"), 7.0, 361, 82, 0, 20, 4.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword2"), 7.0, 361, 82, 0, 20, 3.5, 2.5, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword2"), 5.6, 361, 82, 0, 20, 2.8, 8.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
}

// SPECIAL AIR HI START
unsafe extern "C" fn link_specialairhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_START, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// SPECIAL AIR HI START EFFECT
unsafe extern "C" fn link_effect_specialairhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bound_smoke"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 0.0, 0.0, 0, 0, 0, 1.2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 0.0, 0.0, 0, 0, 0, 1.2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), 9.5, 10.0, 0.0, 0, 0, 0, 1.2, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("link_entry"), Hash40::new("trans"), -9.5, 10.0, 0.0, 0, 0, 0, 1.2, false);
    }
    for _ in 0..6 { 
        if macros::is_excute(agent) { 
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("throw"), 0, 0, 0.0, 90, 0, 0, 1.0, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.8);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

// SPECIAL AIR HI START SOUND
unsafe extern "C" fn link_sound_specialairhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
       macros::PLAY_SE(agent, Hash40::new("se_link_glider_wind"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
       macros::PLAY_SE(agent, Hash40::new("se_link_glider_start"));
    }
}

// SPECIAL AIR HI START EXPRESSION
unsafe extern "C" fn link_expression_specialairhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}

// SPECIAL AIR HI GLIDE
unsafe extern "C" fn link_specialairhiglide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_GLIDE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// SPECIAL AIR HI GLIDE EFFECT
unsafe extern "C" fn link_effect_specialairhiglide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        
    }
}

// SPECIAL AIR HI GLIDE SOUND
unsafe extern "C" fn link_sound_specialairhiglide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        
    }
}

// SPECIAL AIR HI GLIDE EXPRESSION
unsafe extern "C" fn link_expression_specialairhiglide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}

// SPECIAL AIR HI UNEQUIP
unsafe extern "C" fn link_specialairhiunequip(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) { 
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// SPECIAL AIR HI UNEQUIP SOUND
unsafe extern "C" fn link_sound_specialairhiunequip(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
       macros::PLAY_SE(agent, Hash40::new("se_link_glider_unequip"));
    }
}

// SPECIAL AIR HI UNEQUIP EXPRESSION
unsafe extern "C" fn link_expression_specialairhiunequip(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
}

// SPECIAL AIR HI EQUIP
unsafe extern "C" fn link_specialairhiequip(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) { 
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_EQUIP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// SPECIAL AIR HI EQUIP SOUND
unsafe extern "C" fn link_sound_specialairhiequip(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
       macros::PLAY_SE(agent, Hash40::new("se_link_glider_equip"));
    }
}

// SPECIAL AIR HI EQUIP EXPRESSION
unsafe extern "C" fn link_expression_specialairhiequip(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) { 
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}

// SPECIAL AIR HI LANDING
unsafe extern "C" fn link_specialairhilanding(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) { 
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("link")
        .game_acmd("game_specialnstart", link_specialnstart, Low)

        .game_acmd("game_specialhi", link_specialhi, Low)

        .game_acmd("game_specialairhistart", link_specialairhistart, Low)
        .effect_acmd("effect_specialairhistart", link_effect_specialairhistart, Low)
        .sound_acmd("sound_specialairhistart", link_sound_specialairhistart, Low)
        .expression_acmd("expression_specialairhistart", link_expression_specialairhistart, Low)

        .game_acmd("game_specialairhiglide", link_specialairhiglide, Low)
        .effect_acmd("effect_specialairhiglide", link_effect_specialairhiglide, Low)
        .sound_acmd("sound_specialairhiglide", link_sound_specialairhiglide, Low)
        .expression_acmd("expression_specialairhiglide", link_expression_specialairhiglide, Low)

        .game_acmd("game_specialairhiunequip", link_specialairhiunequip, Low)
        .sound_acmd("sound_specialairhiunequip", link_sound_specialairhiunequip, Low)
        .expression_acmd("expression_specialairhiunequip", link_expression_specialairhiunequip, Low)

        .game_acmd("game_specialairhiequip", link_specialairhiequip, Low)
        .sound_acmd("sound_specialairhiequip", link_sound_specialairhiequip, Low)
        .expression_acmd("expression_specialairhiequip", link_expression_specialairhiequip, Low)

        .game_acmd("game_specialairhilanding", link_specialairhilanding, Low)

        .install();
}