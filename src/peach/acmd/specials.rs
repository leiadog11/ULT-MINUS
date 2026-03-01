use super::*;

//--------------------SPECIALS-----------------------

// UP B OPEN
unsafe extern "C" fn peach_specialhiopen(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_open"), false, -1.0);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 2.0, 80, 15, 0, 65, 2.0, -3.1, 6.5, 0.0, Some(3.1), Some(6.5), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 18, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PEACH_PARASOL, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
}

// UP B FALL
unsafe extern "C" fn peach_specialhifall(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_fall"), false, -1.0);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 2.0, 80, 15, 0, 65, 2.0, -3.1, 6.5, 0.0, Some(3.1), Some(6.5), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PEACH_PARASOL, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
}

pub fn install() {
    Agent::new("peach")
        .game_acmd("game_specialhiopen", peach_specialhiopen, Low)
        .game_acmd("game_specialhifall", peach_specialhifall, Low)
        
        .install();
}