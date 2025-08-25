use super::*;

// --------------- WICKED WEAVE LEG ------------------

// UP SMASH
unsafe extern "C" fn wickedweaveleg_attackhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_show") as i64);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 86, 90, 0, 32, 10.5, 0.0, 10.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 86, 90, 0, 32, 10.5, 0.0, 10.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 86, 90, 0, 32, 10.5, 0.0, 20.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 86, 90, 0, 32, 10.5, 0.0, 20.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 86, 90, 0, 32, 10.5, 0.0, 30.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 86, 90, 0, 32, 10.5, 0.0, 30.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 86, 90, 0, 32, 10.5, 0.0, 10.0, 28.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 17.0, 86, 90, 0, 32, 10.5, 0.0, 10.0, 28.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

// UP SMASH EFFECT
unsafe extern "C" fn wickedweaveleg_effect_attackhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_wickedweave_appearance"), Hash40::new("rot"), 0, 0, 0, 0, -60, -90, 1, true);
    }
    if macros::is_excute(agent) {
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.037, 0.04, 0.039);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_wickedweave_line"), Hash40::new("rot"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("bayonetta_wickedweave_hiar1"), Hash40::new("rot"), 0, 0, 0, 0, 0, -90, 1, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.037, 0.04, 0.039);
    }
}

// UP SMASH SOUND
unsafe extern "C" fn wickedweaveleg_sound_attackhi4(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_bayonetta_smash_h02"));
    }
    wait(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_bayonetta_smash_h03"));
    }
}

// FORWARD SMASH
unsafe extern "C" fn wickedweaveleg_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_show") as i64);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 102, 0, 34, 7.5, 0.0, 16.0, 7.0, Some(0.0), Some(12.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 361, 102, 0, 34, 7.5, 0.0, 16.0, 7.0, Some(0.0), Some(12.0), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 16.0, 361, 102, 0, 35, 10.0, 0.0, 10.0, 29.0, None, None, Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 16.0, 361, 102, 0, 35, 10.0, 0.0, 10.0, 29.0, None, None, Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_WICKEDWEAVELEG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// FORWARD SMASH EFFECT
unsafe extern "C" fn wickedweaveleg_effect_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_wickedweave_appearance"), Hash40::new("rot"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    if macros::is_excute(agent) {
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.037, 0.04, 0.039);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("bayonetta_wickedweave_line"), Hash40::new("rot"), 5, -3, 2, -7, 0, -50, 1, true);
    }
}

// FORWARD SMASH SOUND
unsafe extern "C" fn wickedweaveleg_sound_attacks4(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_bayonetta_smash_s02"));
    }
    wait(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_bayonetta_smash_h03"));
    }
}

// DOWN SMASH
unsafe extern "C" fn wickedweaveleg_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_hide") as i64);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_show") as i64);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 72, 90, 0, 10, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 270, 80, 0, 10, 9.0, 0.0, 28.0, 16.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        WorkModule::on_flag(agent.module_accessor, *WEAPON_BAYONETTA_WICKEDWEAVELEG_INSTANCE_WORK_ID_FLAG_DISABLE_LINK_STOP);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 72, 90, 0, 10, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 270, 80, 0, 10, 12.0, 0.0, 8.0, 18.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("bayonetta_wickedweaveleg")
        .game_acmd("game_attackhi4", wickedweaveleg_attackhi4, Low)
        .effect_acmd("effect_attackhi4", wickedweaveleg_effect_attackhi4, Low)
        .sound_acmd("sound_attackhi4", wickedweaveleg_sound_attackhi4, Low)

        .game_acmd("game_attacks4", wickedweaveleg_attacks4, Low)
        .effect_acmd("effect_attacks4", wickedweaveleg_effect_attacks4, Low)
        .sound_acmd("sound_attacks4", wickedweaveleg_sound_attacks4, Low)

        .game_acmd("game_attacklw4", wickedweaveleg_attacklw4, Low)

        .install()
}