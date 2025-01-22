use super::*;

//-------------------SPECIALS------------------

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
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        
    }
}

// SPECIAL AIR HI START SOUND
unsafe extern "C" fn link_sound_specialairhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_link_jump01"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) { 
        macros::PLAY_SE(agent, Hash40::new("se_link_appear01"));
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
        macros::PLAY_SE(agent, Hash40::new("se_link_appear01"));
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
        macros::PLAY_SE(agent, Hash40::new("se_link_appear01"));
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