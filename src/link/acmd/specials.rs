use super::*;

//-------------------SPECIALS------------------

// SPECIAL AIR HI START
unsafe extern "C" fn link_specialairhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, WEAPON_LINK_PARASAIL_STATUS_KIND_SPECIAL_AIR_HI_START, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// SPECIAL AIR HI START EFFECT
unsafe extern "C" fn link_effect_specialairhistart(agent: &mut L2CAgentBase) {
    
}

// SPECIAL AIR HI START SOUND
unsafe extern "C" fn link_sound_specialairhistart(agent: &mut L2CAgentBase) {
    
}

// SPECIAL AIR HI START EXPRESSION
unsafe extern "C" fn link_expression_specialairhi(agent: &mut L2CAgentBase) {
    
}

// SPECIAL AIR HI UNEQUIP
unsafe extern "C" fn link_specialairhiunequip(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// SPECIAL AIR HI LANDING
unsafe extern "C" fn link_specialairhilanding(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    Agent::new("link")
        .game_acmd("game_specialairhistart", link_specialairhistart, Low)
        .effect_acmd("effect_specialairhistart", link_effect_specialairhistart, Low)
        .sound_acmd("sound_specialairhistart", link_sound_specialairhistart, Low)
        .expression_acmd("expression_specialairhistart", link_expression_specialairhi, Low)

        .game_acmd("game_specialairhiunequip", link_specialairhiunequip, Low)

        .game_acmd("game_specialairhilanding", link_specialairhilanding, Low)

        .install();
}